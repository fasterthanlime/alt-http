#![allow(incomplete_features)]
#![allow(unused_attributes)]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]

use hring::{h1, Body, BodyChunk, Response, RollMut, WriteOwned};
use std::{cell::RefCell, future::Future, net::SocketAddr, rc::Rc};
use tracing::debug;

pub type TransportPool = Rc<RefCell<Vec<Rc<tokio_uring::net::TcpStream>>>>;

pub struct ProxyDriver {
    pub upstream_addr: SocketAddr,
    pub pool: TransportPool,
}

impl h1::ServerDriver for ProxyDriver {
    async fn handle<T: WriteOwned>(
        &self,
        req: hring::Request,
        req_body: &mut impl Body,
        respond: h1::Responder<T, h1::ExpectResponseHeaders>,
    ) -> eyre::Result<h1::Responder<T, h1::ResponseDone>> {
        let transport = {
            let mut pool = self.pool.borrow_mut();
            pool.pop()
        };

        let transport = if let Some(transport) = transport {
            debug!("re-using existing transport!");
            transport
        } else {
            debug!("making new connection to upstream!");
            Rc::new(tokio_uring::net::TcpStream::connect(self.upstream_addr).await?)
        };

        let driver = ProxyClientDriver { respond };

        let (transport, res) = h1::request(transport, req, req_body, driver).await?;

        if let Some(transport) = transport {
            let mut pool = self.pool.borrow_mut();
            pool.push(transport);
        }

        Ok(res)
    }
}

struct ProxyClientDriver<T>
where
    T: WriteOwned,
{
    respond: h1::Responder<T, h1::ExpectResponseHeaders>,
}

impl<T> h1::ClientDriver for ProxyClientDriver<T>
where
    T: WriteOwned,
{
    type Return = h1::Responder<T, h1::ResponseDone>;

    async fn on_informational_response(&self, res: Response) -> eyre::Result<()> {
        debug!("Got informational response {}", res.status);
        Ok(())
    }

    async fn on_final_response(
        self,
        res: Response,
        body: &mut impl Body,
    ) -> eyre::Result<Self::Return> {
        let respond = self.respond;
        let mut respond = respond.write_final_response(res).await?;

        let trailers = loop {
            match body.next_chunk().await? {
                BodyChunk::Chunk(chunk) => {
                    respond = respond.write_chunk(chunk).await?;
                }
                BodyChunk::Done { trailers } => {
                    // should we do something here in case of
                    // content-length mismatches or something?
                    break trailers;
                }
            }
        };

        let respond = respond.finish_body(trailers).await?;

        Ok(respond)
    }
}

pub async fn start(
    upstream_addr: SocketAddr,
) -> eyre::Result<(
    SocketAddr,
    impl Drop,
    impl Future<Output = eyre::Result<()>>,
)> {
    let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();

    let ln = tokio_uring::net::TcpListener::bind("[::]:0".parse()?)?;
    let ln_addr = ln.local_addr()?;

    let proxy_fut = async move {
        let conf = Rc::new(h1::ServerConf::default());
        let pool: TransportPool = Default::default();

        enum Event {
            Accepted((tokio_uring::net::TcpStream, SocketAddr)),
            ShuttingDown,
        }

        loop {
            let ev = tokio::select! {
                accept_res = ln.accept() => {
                    Event::Accepted(accept_res?)
                },
                _ = &mut rx => {
                    Event::ShuttingDown
                }
            };

            match ev {
                Event::Accepted((transport, remote_addr)) => {
                    debug!("Accepted connection from {remote_addr}");

                    let pool = pool.clone();
                    let conf = conf.clone();

                    tokio_uring::spawn(async move {
                        let driver = ProxyDriver {
                            upstream_addr,
                            pool,
                        };
                        h1::serve(transport, conf, RollMut::alloc().unwrap(), driver)
                            .await
                            .unwrap();
                        debug!("Done serving h1 connection");
                    });
                }
                Event::ShuttingDown => {
                    debug!("Shutting down proxy");
                    break;
                }
            }
        }

        debug!("Proxy server shutting down.");
        drop(pool);

        Ok(())
    };

    Ok((ln_addr, tx, proxy_fut))
}
