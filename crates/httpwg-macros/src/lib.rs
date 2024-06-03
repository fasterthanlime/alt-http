//! Macros to help generate code for all suites/groups/tests of the httpwg crate

// This file is automatically @generated by httpwg-gen
// It is not intended for manual editing

/// This generates a module tree with some #[test] functions.
/// The `$body` argument is pasted inside those unit test, and
/// in that scope, `test` is the `httpwg` function you can use
/// to run the test (that takes a `mut conn: Conn<IO>`)
#[macro_export]
macro_rules! tests {
  ($body: tt) => {

/// RFC 9113 describes an optimized expression of the
/// semantics of the Hypertext Transfer Protocol (HTTP), referred to as
/// HTTP version 2 (HTTP/2).
///
/// HTTP/2 enables a more efficient use of network resources and a reduced
/// latency by introducing field compression and allowing multiple concurrent
/// exchanges on the same connection.
///
/// This document obsoletes RFCs 7540 and 8740.
///
/// cf. <https://httpwg.org/specs/rfc9113.html>
#[cfg(test)]
mod rfc9113 {
use ::httpwg::rfc9113 as __suite;

/// Section 3: Starting HTTP/2
mod _3_starting_http2 {
use super::__suite::_3_starting_http2 as __group;

/// The server connection preface consists of a potentially empty
/// SETTINGS frame (Section 6.5) that MUST be the first frame
/// the server sends in the HTTP/2 connection.
#[test]
fn sends_client_connection_preface() {
use __group::sends_client_connection_preface as test;
$body
}

/// Clients and servers MUST treat an invalid connection preface as
/// a connection error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_invalid_connection_preface() {
use __group::sends_invalid_connection_preface as test;
$body
}
}

/// Section 4: HTTP Frames
mod _4_http_frames {
use super::__suite::_4_http_frames as __group;

/// Implementations MUST ignore and discard frames of unknown types.
#[test]
fn sends_frame_with_unknown_type() {
use __group::sends_frame_with_unknown_type as test;
$body
}

/// Unused flags MUST be ignored on receipt and MUST be left
/// unset (0x00) when sending.
#[test]
fn sends_frame_with_unused_flags() {
use __group::sends_frame_with_unused_flags as test;
$body
}

/// Reserved: A reserved 1-bit field. The semantics of this bit are
/// undefined, and the bit MUST remain unset (0x00) when sending and
/// MUST be ignored when receiving.
#[test]
fn sends_frame_with_reserved_bit_set() {
use __group::sends_frame_with_reserved_bit_set as test;
$body
}

#[test]
fn data_frame_with_max_length() {
use __group::data_frame_with_max_length as test;
$body
}

/// An endpoint MUST send an error code of FRAME_SIZE_ERROR if a frame
/// exceeds the size defined in SETTINGS_MAX_FRAME_SIZE, exceeds any
/// limit defined for the frame type, or is too small to contain mandatory frame
/// data
#[test]
fn frame_exceeding_max_size() {
use __group::frame_exceeding_max_size as test;
$body
}

/// A frame size error in a frame that could alter the state of
/// the entire connection MUST be treated as a connection error
/// (Section 5.4.1); this includes any frame carrying a field block
/// (Section 4.3) (that is, HEADERS, PUSH_PROMISE, and CONTINUATION),
/// a SETTINGS frame, and any frame with a stream identifier of 0.
#[test]
fn large_headers_frame_exceeding_max_size() {
use __group::large_headers_frame_exceeding_max_size as test;
$body
}

/// A decoding error in a header block MUST be treated as a connection error
/// (Section 5.4.1) of type COMPRESSION_ERROR.
#[test]
fn invalid_header_block_fragment() {
use __group::invalid_header_block_fragment as test;
$body
}

/// Each header block is processed as a discrete unit. Header blocks
/// MUST be transmitted as a contiguous sequence of frames, with no
/// interleaved frames of any other type or from any other stream.
#[test]
fn priority_frame_while_sending_headers() {
use __group::priority_frame_while_sending_headers as test;
$body
}

/// Each header block is processed as a discrete unit. Header blocks
/// MUST be transmitted as a contiguous sequence of frames, with no
/// interleaved frames of any other type or from any other stream.
#[test]
fn headers_frame_to_another_stream() {
use __group::headers_frame_to_another_stream as test;
$body
}
}

/// Section 5: Streams and Multiplexing
mod _5_streams_and_multiplexing {
use super::__suite::_5_streams_and_multiplexing as __group;

/// idle:
/// Receiving any frame other than HEADERS or PRIORITY on a stream
/// in this state MUST be treated as a connection error
/// (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn idle_sends_data_frame() {
use __group::idle_sends_data_frame as test;
$body
}

/// idle:
/// Receiving any frame other than HEADERS or PRIORITY on a stream
/// in this state MUST be treated as a connection error
/// (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn idle_sends_rst_stream_frame() {
use __group::idle_sends_rst_stream_frame as test;
$body
}

/// idle:
/// Receiving any frame other than HEADERS or PRIORITY on a stream
/// in this state MUST be treated as a connection error
/// (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn idle_sends_window_update_frame() {
use __group::idle_sends_window_update_frame as test;
$body
}

/// idle:
/// Receiving any frame other than HEADERS or PRIORITY on a stream
/// in this state MUST be treated as a connection error
/// (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn idle_sends_continuation_frame() {
use __group::idle_sends_continuation_frame as test;
$body
}

/// half-closed (remote):
/// If an endpoint receives additional frames, other than
/// WINDOW_UPDATE, PRIORITY, or RST_STREAM, for a stream that is in
/// this state, it MUST respond with a stream error (Section 5.4.2)
/// of type STREAM_CLOSED.
#[test]
fn half_closed_remote_sends_data_frame() {
use __group::half_closed_remote_sends_data_frame as test;
$body
}

/// half-closed (remote):
/// If an endpoint receives additional frames, other than
/// WINDOW_UPDATE, PRIORITY, or RST_STREAM, for a stream that is in
/// this state, it MUST respond with a stream error (Section 5.4.2)
/// of type STREAM_CLOSED.
#[test]
fn half_closed_remote_sends_headers_frame() {
use __group::half_closed_remote_sends_headers_frame as test;
$body
}

/// half-closed (remote):
/// If an endpoint receives additional frames, other than
/// WINDOW_UPDATE, PRIORITY, or RST_STREAM, for a stream that is in
/// this state, it MUST respond with a stream error (Section 5.4.2)
/// of type STREAM_CLOSED.
#[test]
fn half_closed_remote_sends_continuation_frame() {
use __group::half_closed_remote_sends_continuation_frame as test;
$body
}

/// closed:
/// An endpoint that receives any frame other than PRIORITY after
/// receiving a RST_STREAM MUST treat that as a stream error
/// (Section 5.4.2) of type STREAM_CLOSED.
#[test]
fn closed_sends_data_frame_after_rst_stream() {
use __group::closed_sends_data_frame_after_rst_stream as test;
$body
}

/// closed:
/// An endpoint that receives any frame other than PRIORITY after
/// receiving a RST_STREAM MUST treat that as a stream error
/// (Section 5.4.2) of type STREAM_CLOSED.
#[test]
fn closed_sends_headers_frame_after_rst_stream() {
use __group::closed_sends_headers_frame_after_rst_stream as test;
$body
}

/// closed:
/// An endpoint that receives any frame other than PRIORITY after
/// receiving a RST_STREAM MUST treat that as a stream error
/// (Section 5.4.2) of type STREAM_CLOSED.
#[test]
fn closed_sends_continuation_frame_after_rst_stream() {
use __group::closed_sends_continuation_frame_after_rst_stream as test;
$body
}

/// closed:
/// An endpoint that receives any frames after receiving a frame
/// with the END_STREAM flag set MUST treat that as a connection
/// error (Section 6.4.1) of type STREAM_CLOSED.
#[test]
fn closed_sends_data_frame() {
use __group::closed_sends_data_frame as test;
$body
}

/// closed:
/// An endpoint that receives any frames after receiving a frame
/// with the END_STREAM flag set MUST treat that as a connection
/// error (Section 6.4.1) of type STREAM_CLOSED.
#[test]
fn closed_sends_headers_frame() {
use __group::closed_sends_headers_frame as test;
$body
}

/// closed:
/// An endpoint that receives any frames after receiving a frame
/// with the END_STREAM flag set MUST treat that as a connection
/// error (Section 6.4.1) of type STREAM_CLOSED.
#[test]
fn closed_sends_continuation_frame() {
use __group::closed_sends_continuation_frame as test;
$body
}

/// An endpoint that receives an unexpected stream identifier
/// MUST respond with a connection error (Section 5.4.1) of
/// type PROTOCOL_ERROR.
#[test]
fn sends_even_numbered_stream_identifier() {
use __group::sends_even_numbered_stream_identifier as test;
$body
}

/// An endpoint that receives an unexpected stream identifier
/// MUST respond with a connection error (Section 5.4.1) of
/// type PROTOCOL_ERROR.
#[test]
fn sends_smaller_stream_identifier() {
use __group::sends_smaller_stream_identifier as test;
$body
}

#[test]
fn exceeds_concurrent_stream_limit() {
use __group::exceeds_concurrent_stream_limit as test;
$body
}

/// After sending the GOAWAY frame for an error condition,
/// the endpoint MUST close the TCP connection.
#[test]
fn invalid_ping_frame_for_connection_close() {
use __group::invalid_ping_frame_for_connection_close as test;
$body
}

#[test]
fn test_invalid_ping_frame_for_goaway() {
use __group::test_invalid_ping_frame_for_goaway as test;
$body
}

/// Extension frames that appear in the middle of a header block
/// (Section 4.3) are not permitted; these MUST be treated as
/// a connection error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn unknown_extension_frame_in_header_block() {
use __group::unknown_extension_frame_in_header_block as test;
$body
}
}

/// Section 6: Frame Definitions
mod _6_frame_definitions {
use super::__suite::_6_frame_definitions as __group;

/// DATA frames MUST be associated with a stream. If a DATA frame is
/// received whose stream identifier field is 0x0, the recipient
/// MUST respond with a connection error (Section 5.4.1) of type
/// PROTOCOL_ERROR.
#[test]
fn sends_data_frame_with_zero_stream_id() {
use __group::sends_data_frame_with_zero_stream_id as test;
$body
}

/// If a DATA frame is received whose stream is not in "open" or
/// "half-closed (local)" state, the recipient MUST respond with
/// a stream error (Section 5.4.2) of type STREAM_CLOSED.
///
/// Note: This test case is duplicated with 5.1.
#[test]
fn sends_data_frame_on_invalid_stream_state() {
use __group::sends_data_frame_on_invalid_stream_state as test;
$body
}

/// If the length of the padding is the length of the frame payload
/// or greater, the recipient MUST treat this as a connection error
/// (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_data_frame_with_invalid_pad_length() {
use __group::sends_data_frame_with_invalid_pad_length as test;
$body
}

/// HEADERS frames MUST be associated with a stream. If a HEADERS
/// frame is received whose stream identifier field is 0x0, the
/// recipient MUST respond with a connection error (Section 5.4.1)
/// of type PROTOCOL_ERROR.
#[test]
fn sends_headers_frame_with_zero_stream_id() {
use __group::sends_headers_frame_with_zero_stream_id as test;
$body
}

/// The HEADERS frame can include padding. Padding fields and flags
/// are identical to those defined for DATA frames (Section 6.1).
/// Padding that exceeds the size remaining for the header block
/// fragment MUST be treated as a PROTOCOL_ERROR.
#[test]
fn sends_headers_frame_with_invalid_pad_length() {
use __group::sends_headers_frame_with_invalid_pad_length as test;
$body
}

/// The PRIORITY frame always identifies a stream. If a PRIORITY
/// frame is received with a stream identifier of 0x0, the recipient
/// MUST respond with a connection error (Section 5.4.1) of type
/// PROTOCOL_ERROR.
#[test]
fn sends_priority_frame_with_zero_stream_id() {
use __group::sends_priority_frame_with_zero_stream_id as test;
$body
}

/// A PRIORITY frame with a length other than 5 octets MUST be
/// treated as a stream error (Section 5.4.2) of type
/// FRAME_SIZE_ERROR.
#[test]
fn sends_priority_frame_with_invalid_length() {
use __group::sends_priority_frame_with_invalid_length as test;
$body
}

/// RST_STREAM frames MUST be associated with a stream. If a
/// RST_STREAM frame is received with a stream identifier of 0x0,
/// the recipient MUST treat this as a connection error
/// (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_rst_stream_frame_with_zero_stream_id() {
use __group::sends_rst_stream_frame_with_zero_stream_id as test;
$body
}

/// RST_STREAM frames MUST NOT be sent for a stream in the "idle"
/// state. If a RST_STREAM frame identifying an idle stream is
/// received, the recipient MUST treat this as a connection error
/// (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_rst_stream_frame_on_idle_stream() {
use __group::sends_rst_stream_frame_on_idle_stream as test;
$body
}

/// A RST_STREAM frame with a length other than 4 octets MUST be
/// treated as a connection error (Section 5.4.1) of type
/// FRAME_SIZE_ERROR.
#[test]
fn sends_rst_stream_frame_with_invalid_length() {
use __group::sends_rst_stream_frame_with_invalid_length as test;
$body
}

/// ACK (0x1):
/// When set, bit 0 indicates that this frame acknowledges receipt
/// and application of the peer's SETTINGS frame. When this bit is
/// set, the payload of the SETTINGS frame MUST be empty. Receipt of
/// a SETTINGS frame with the ACK flag set and a length field value
/// other than 0 MUST be treated as a connection error (Section 5.4.1)
/// of type FRAME_SIZE_ERROR.
#[test]
fn sends_settings_frame_with_ack_and_payload() {
use __group::sends_settings_frame_with_ack_and_payload as test;
$body
}

/// SETTINGS frames always apply to a connection, never a single
/// stream. The stream identifier for a SETTINGS frame MUST be
/// zero (0x0). If an endpoint receives a SETTINGS frame whose
/// stream identifier field is anything other than 0x0, the
/// endpoint MUST respond with a connection error (Section 5.4.1)
/// of type PROTOCOL_ERROR.
#[test]
fn sends_settings_frame_with_non_zero_stream_id() {
use __group::sends_settings_frame_with_non_zero_stream_id as test;
$body
}

/// The SETTINGS frame affects connection state. A badly formed or
/// incomplete SETTINGS frame MUST be treated as a connection error
/// (Section 5.4.1) of type PROTOCOL_ERROR.
///
/// A SETTINGS frame with a length other than a multiple of 6 octets
/// MUST be treated as a connection error (Section 5.4.1) of type
/// FRAME_SIZE_ERROR.
#[test]
fn sends_settings_frame_with_invalid_length() {
use __group::sends_settings_frame_with_invalid_length as test;
$body
}

/// SETTINGS_ENABLE_PUSH (0x2):
/// The initial value is 1, which indicates that server push is
/// permitted. Any value other than 0 or 1 MUST be treated as a
/// connection error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_settings_enable_push_with_invalid_value() {
use __group::sends_settings_enable_push_with_invalid_value as test;
$body
}

/// SETTINGS_INITIAL_WINDOW_SIZE (0x4):
/// Values above the maximum flow-control window size of 2^31-1
/// MUST be treated as a connection error (Section 5.4.1) of
/// type FLOW_CONTROL_ERROR.
#[test]
fn sends_settings_initial_window_size_with_invalid_value() {
use __group::sends_settings_initial_window_size_with_invalid_value as test;
$body
}

/// SETTINGS_MAX_FRAME_SIZE (0x5):
/// The initial value is 2^14 (16,384) octets. The value advertised
/// by an endpoint MUST be between this initial value and the
/// maximum allowed frame size (2^24-1 or 16,777,215 octets),
/// inclusive. Values outside this range MUST be treated as a
/// connection error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_settings_max_frame_size_with_invalid_value_below_initial() {
use __group::sends_settings_max_frame_size_with_invalid_value_below_initial as test;
$body
}

/// SETTINGS_MAX_FRAME_SIZE (0x5):
/// The initial value is 2^14 (16,384) octets. The value advertised
/// by an endpoint MUST be between this initial value and the
/// maximum allowed frame size (2^24-1 or 16,777,215 octets),
/// inclusive. Values outside this range MUST be treated as a
/// connection error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_settings_max_frame_size_with_invalid_value_above_max() {
use __group::sends_settings_max_frame_size_with_invalid_value_above_max as test;
$body
}

/// An endpoint that receives a SETTINGS frame with any unknown
/// or unsupported identifier MUST ignore that setting.
#[test]
fn sends_settings_frame_with_unknown_identifier() {
use __group::sends_settings_frame_with_unknown_identifier as test;
$body
}

/// The values in the SETTINGS frame MUST be processed in the order
/// they appear, with no other frame processing between values.
#[test]
fn sends_multiple_values_of_settings_initial_window_size() {
use __group::sends_multiple_values_of_settings_initial_window_size as test;
$body
}

/// Once all values have been processed, the recipient MUST
/// immediately emit a SETTINGS frame with the ACK flag set.
#[test]
fn sends_settings_frame_without_ack_flag() {
use __group::sends_settings_frame_without_ack_flag as test;
$body
}

/// Receivers of a PING frame that does not include an ACK flag MUST
/// send a PING frame with the ACK flag set in response, with an
/// identical payload.
#[test]
fn sends_ping_frame() {
use __group::sends_ping_frame as test;
$body
}

/// ACK (0x1):
/// When set, bit 0 indicates that this PING frame is a PING
/// response. An endpoint MUST set this flag in PING responses.
/// An endpoint MUST NOT respond to PING frames containing this
/// flag.
#[test]
fn sends_ping_frame_with_ack() {
use __group::sends_ping_frame_with_ack as test;
$body
}

/// If a PING frame is received with a stream identifier field value
/// other than 0x0, the recipient MUST respond with a connection
/// error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_ping_frame_with_non_zero_stream_id() {
use __group::sends_ping_frame_with_non_zero_stream_id as test;
$body
}

/// Receipt of a PING frame with a length field value other than 8
/// MUST be treated as a connection error (Section 5.4.1) of type
/// FRAME_SIZE_ERROR.
#[test]
fn sends_ping_frame_with_invalid_length() {
use __group::sends_ping_frame_with_invalid_length as test;
$body
}

/// An endpoint MUST treat a GOAWAY frame with a stream identifier
/// other than 0x0 as a connection error (Section 5.4.1) of type
/// PROTOCOL_ERROR.
#[test]
fn sends_goaway_frame_with_non_zero_stream_id() {
use __group::sends_goaway_frame_with_non_zero_stream_id as test;
$body
}

/// A receiver MUST treat the receipt of a WINDOW_UPDATE frame with
/// a flow-control window increment of 0 as a stream error
/// (Section 5.4.2) of type PROTOCOL_ERROR; errors on the connection
/// flow-control window MUST be treated as a connection error
/// (Section 5.4.1).
#[test]
fn sends_window_update_frame_with_zero_increment() {
use __group::sends_window_update_frame_with_zero_increment as test;
$body
}

/// A receiver MUST treat the receipt of a WINDOW_UPDATE frame with
/// a flow-control window increment of 0 as a stream error
/// (Section 5.4.2) of type PROTOCOL_ERROR; errors on the connection
/// flow-control window MUST be treated as a connection error
/// (Section 5.4.1).
#[test]
fn sends_window_update_frame_with_zero_increment_on_stream() {
use __group::sends_window_update_frame_with_zero_increment_on_stream as test;
$body
}

/// A WINDOW_UPDATE frame with a length other than 4 octets MUST
/// be treated as a connection error (Section 5.4.1) of type
/// FRAME_SIZE_ERROR.
#[test]
fn sends_window_update_frame_with_invalid_length() {
use __group::sends_window_update_frame_with_invalid_length as test;
$body
}

/// The sender MUST NOT send a flow-controlled frame with a length
/// that exceeds the space available in either of the flow-control
/// windows advertised by the receiver.
#[test]
fn sends_settings_frame_to_set_initial_window_size_to_1_and_sends_headers_frame() {
use __group::sends_settings_frame_to_set_initial_window_size_to_1_and_sends_headers_frame as test;
$body
}

/// A sender MUST NOT allow a flow-control window to exceed 2^31-1
/// octets. If a sender receives a WINDOW_UPDATE that causes a
/// flow-control window to exceed this maximum, it MUST terminate
/// either the stream or the connection, as appropriate.
/// For streams, the sender sends a RST_STREAM with an error code
/// of FLOW_CONTROL_ERROR; for the connection, a GOAWAY frame with
/// an error code of FLOW_CONTROL_ERROR is sent.
#[test]
fn sends_multiple_window_update_frames_increasing_flow_control_window_above_max() {
use __group::sends_multiple_window_update_frames_increasing_flow_control_window_above_max as test;
$body
}

/// A sender MUST NOT allow a flow-control window to exceed 2^31-1
/// octets. If a sender receives a WINDOW_UPDATE that causes a
/// flow-control window to exceed this maximum, it MUST terminate
/// either the stream or the connection, as appropriate.
/// For streams, the sender sends a RST_STREAM with an error code
/// of FLOW_CONTROL_ERROR; for the connection, a GOAWAY frame with
/// an error code of FLOW_CONTROL_ERROR is sent.
#[test]
fn sends_multiple_window_update_frames_increasing_flow_control_window_above_max_on_stream() {
use __group::sends_multiple_window_update_frames_increasing_flow_control_window_above_max_on_stream as test;
$body
}

/// When the value of SETTINGS_INITIAL_WINDOW_SIZE changes,
/// a receiver MUST adjust the size of all stream flow-control
/// windows that it maintains by the difference between the new
/// value and the old value.
#[test]
fn changes_settings_initial_window_size_after_sending_headers_frame() {
use __group::changes_settings_initial_window_size_after_sending_headers_frame as test;
$body
}

/// A sender MUST track the negative flow-control window and
/// MUST NOT send new flow-controlled frames until it receives
/// WINDOW_UPDATE frames that cause the flow-control window to
/// become positive.
#[test]
fn sends_settings_frame_for_window_size_to_be_negative() {
use __group::sends_settings_frame_for_window_size_to_be_negative as test;
$body
}

/// An endpoint MUST treat a change to SETTINGS_INITIAL_WINDOW_SIZE
/// that causes any flow-control window to exceed the maximum size
/// as a connection error (Section 5.4.1) of type FLOW_CONTROL_ERROR.
#[test]
fn sends_settings_initial_window_size_with_exceeded_max_window_size_value() {
use __group::sends_settings_initial_window_size_with_exceeded_max_window_size_value as test;
$body
}

/// The CONTINUATION frame (type=0x9) is used to continue a sequence
/// of header block fragments (Section 4.3). Any number of
/// CONTINUATION frames can be sent, as long as the preceding frame
/// is on the same stream and is a HEADERS, PUSH_PROMISE,
/// or CONTINUATION frame without the END_HEADERS flag set.
#[test]
fn sends_multiple_continuation_frames_preceded_by_headers_frame() {
use __group::sends_multiple_continuation_frames_preceded_by_headers_frame as test;
$body
}

/// END_HEADERS (0x4):
/// If the END_HEADERS bit is not set, this frame MUST be followed
/// by another CONTINUATION frame. A receiver MUST treat the receipt
/// of any other type of frame or a frame on a different stream as
/// a connection error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_continuation_frame_followed_by_non_continuation_frame() {
use __group::sends_continuation_frame_followed_by_non_continuation_frame as test;
$body
}

/// CONTINUATION frames MUST be associated with a stream. If a
/// CONTINUATION frame is received whose stream identifier field is
/// 0x0, the recipient MUST respond with a connection error
/// (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_continuation_frame_with_zero_stream_id() {
use __group::sends_continuation_frame_with_zero_stream_id as test;
$body
}

/// A CONTINUATION frame MUST be preceded by a HEADERS, PUSH_PROMISE
/// or CONTINUATION frame without the END_HEADERS flag set.
/// A recipient that observes violation of this rule MUST respond
/// with a connection error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_continuation_frame_preceded_by_headers_frame_with_end_headers_flag() {
use __group::sends_continuation_frame_preceded_by_headers_frame_with_end_headers_flag as test;
$body
}

/// A CONTINUATION frame MUST be preceded by a HEADERS, PUSH_PROMISE
/// or CONTINUATION frame without the END_HEADERS flag set.
/// A recipient that observes violation of this rule MUST respond
/// with a connection error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_continuation_frame_preceded_by_continuation_frame_with_end_headers_flag() {
use __group::sends_continuation_frame_preceded_by_continuation_frame_with_end_headers_flag as test;
$body
}

/// A CONTINUATION frame MUST be preceded by a HEADERS, PUSH_PROMISE
/// or CONTINUATION frame without the END_HEADERS flag set.
/// A recipient that observes violation of this rule MUST respond
/// with a connection error (Section 5.4.1) of type PROTOCOL_ERROR.
#[test]
fn sends_continuation_frame_preceded_by_data_frame() {
use __group::sends_continuation_frame_preceded_by_data_frame as test;
$body
}
}

/// Section 7: Error Codes
mod _7_error_codes {
use super::__suite::_7_error_codes as __group;

/// Unknown or unsupported error codes MUST NOT trigger any special
/// behavior. These MAY be treated by an implementation as being
/// equivalent to INTERNAL_ERROR.
#[test]
fn sends_goaway_frame_with_unknown_error_code() {
use __group::sends_goaway_frame_with_unknown_error_code as test;
$body
}

/// Unknown or unsupported error codes MUST NOT trigger any special
/// behavior. These MAY be treated by an implementation as being
/// equivalent to INTERNAL_ERROR.
#[test]
fn sends_rst_stream_frame_with_unknown_error_code() {
use __group::sends_rst_stream_frame_with_unknown_error_code as test;
$body
}
}

/// Section 8: Expressing HTTP Semantics in HTTP/2
mod _8_expressing_http_semantics_in_http2 {
use super::__suite::_8_expressing_http_semantics_in_http2 as __group;

#[test]
fn sends_second_headers_frame_without_end_stream() {
use __group::sends_second_headers_frame_without_end_stream as test;
$body
}

/// A field name MUST NOT contain characters in the ranges 0x00-0x20, 0x41-0x5a,
/// or 0x7f-0xff (all ranges inclusive). This specifically excludes all
/// non-visible ASCII characters, ASCII SP (0x20), and uppercase characters ('A'
/// to 'Z', ASCII 0x41 to 0x5a).
///
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_uppercase_field_name() {
use __group::sends_headers_frame_with_uppercase_field_name as test;
$body
}

/// A field name MUST NOT contain characters in the ranges 0x00-0x20, 0x41-0x5a,
/// or 0x7f-0xff (all ranges inclusive). This specifically excludes all
/// non-visible ASCII characters, ASCII SP (0x20), and uppercase characters ('A'
/// to 'Z', ASCII 0x41 to 0x5a).
///
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_space_in_field_name() {
use __group::sends_headers_frame_with_space_in_field_name as test;
$body
}

/// A field name MUST NOT contain characters in the ranges 0x00-0x20, 0x41-0x5a,
/// or 0x7f-0xff (all ranges inclusive). This specifically excludes all
/// non-visible ASCII characters, ASCII SP (0x20), and uppercase characters ('A'
/// to 'Z', ASCII 0x41 to 0x5a).
///
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_non_visible_ascii() {
use __group::sends_headers_frame_with_non_visible_ascii as test;
$body
}

/// A field name MUST NOT contain characters in the ranges 0x00-0x20, 0x41-0x5a,
/// or 0x7f-0xff (all ranges inclusive). This specifically excludes all
/// non-visible ASCII characters, ASCII SP (0x20), and uppercase characters ('A'
/// to 'Z', ASCII 0x41 to 0x5a).
///
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_del_character() {
use __group::sends_headers_frame_with_del_character as test;
$body
}

/// A field name MUST NOT contain characters in the ranges 0x00-0x20, 0x41-0x5a,
/// or 0x7f-0xff (all ranges inclusive). This specifically excludes all
/// non-visible ASCII characters, ASCII SP (0x20), and uppercase characters ('A'
/// to 'Z', ASCII 0x41 to 0x5a).
///
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_non_ascii_character() {
use __group::sends_headers_frame_with_non_ascii_character as test;
$body
}

/// With the exception of pseudo-header fields (Section 8.3), which have a name
/// that starts with a single colon, field names MUST NOT include a colon (ASCII
/// COLON, 0x3a).
///
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_colon_in_field_name() {
use __group::sends_headers_frame_with_colon_in_field_name as test;
$body
}

/// A field value MUST NOT contain the zero value (ASCII NUL, 0x00), line feed
/// (ASCII LF, 0x0a), or carriage return (ASCII CR, 0x0d) at any position.
///
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_lf_in_field_value() {
use __group::sends_headers_frame_with_lf_in_field_value as test;
$body
}

/// A field value MUST NOT contain the zero value (ASCII NUL, 0x00), line feed
/// (ASCII LF, 0x0a), or carriage return (ASCII CR, 0x0d) at any position.
///
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_cr_in_field_value() {
use __group::sends_headers_frame_with_cr_in_field_value as test;
$body
}

/// A field value MUST NOT contain the zero value (ASCII NUL, 0x00), line feed
/// (ASCII LF, 0x0a), or carriage return (ASCII CR, 0x0d) at any position.
///
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_nul_in_field_value() {
use __group::sends_headers_frame_with_nul_in_field_value as test;
$body
}

/// A field value MUST NOT start or end with an ASCII whitespace character
/// (ASCII SP or HTAB, 0x20 or 0x09).
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_leading_space_in_field_value() {
use __group::sends_headers_frame_with_leading_space_in_field_value as test;
$body
}

/// A field value MUST NOT start or end with an ASCII whitespace character
/// (ASCII SP or HTAB, 0x20 or 0x09).
/// When a request message violates one of these requirements, an implementation
/// SHOULD generate a 400 (Bad Request) status code (see Section 15.5.1 of
/// [HTTP]), unless a more suitable status code is defined or the status code
/// cannot be sent (e.g., because the error occurs in a trailer field).
#[test]
fn sends_headers_frame_with_trailing_tab_in_field_value() {
use __group::sends_headers_frame_with_trailing_tab_in_field_value as test;
$body
}

/// HTTP/2 does not use the Connection header field (Section 7.6.1 of [HTTP]) to
/// indicate connection-specific header fields; in this protocol,
/// connection-specific metadata is conveyed by other means. An endpoint MUST
/// NOT generate an HTTP/2 message containing connection-specific header fields.
/// This includes the Connection header field and those listed as having
/// connection-specific semantics in Section 7.6.1 of [HTTP] (that is,
/// Proxy-Connection, Keep-Alive, Transfer-Encoding, and Upgrade). Any message
/// containing connection-specific header fields MUST be treated as malformed
/// (Section 8.1.1).
#[test]
fn sends_headers_frame_with_connection_header() {
use __group::sends_headers_frame_with_connection_header as test;
$body
}

/// HTTP/2 does not use the Connection header field (Section 7.6.1 of [HTTP]) to
/// indicate connection-specific header fields; in this protocol,
/// connection-specific metadata is conveyed by other means. An endpoint MUST
/// NOT generate an HTTP/2 message containing connection-specific header fields.
///
/// This includes the Connection header field and those listed as having
/// connection-specific semantics in Section 7.6.1 of [HTTP] (that is,
/// Proxy-Connection, Keep-Alive, Transfer-Encoding, and Upgrade). Any message
/// containing connection-specific header fields MUST be treated as malformed
/// (Section 8.1.1).
#[test]
fn sends_headers_frame_with_proxy_connection_header() {
use __group::sends_headers_frame_with_proxy_connection_header as test;
$body
}

/// HTTP/2 does not use the Connection header field (Section 7.6.1 of [HTTP]) to
/// indicate connection-specific header fields; in this protocol,
/// connection-specific metadata is conveyed by other means. An endpoint MUST
/// NOT generate an HTTP/2 message containing connection-specific header fields.
///
/// This includes the Connection header field and those listed as having
/// connection-specific semantics in Section 7.6.1 of [HTTP] (that is,
/// Proxy-Connection, Keep-Alive, Transfer-Encoding, and Upgrade). Any message
/// containing connection-specific header fields MUST be treated as malformed
/// (Section 8.1.1).
#[test]
fn sends_headers_frame_with_keep_alive_header() {
use __group::sends_headers_frame_with_keep_alive_header as test;
$body
}

/// HTTP/2 does not use the Connection header field (Section 7.6.1 of [HTTP]) to
/// indicate connection-specific header fields; in this protocol,
/// connection-specific metadata is conveyed by other means. An endpoint MUST
/// NOT generate an HTTP/2 message containing connection-specific header fields.
///
/// This includes the Connection header field and those listed as having
/// connection-specific semantics in Section 7.6.1 of [HTTP] (that is,
/// Proxy-Connection, Keep-Alive, Transfer-Encoding, and Upgrade). Any message
/// containing connection-specific header fields MUST be treated as malformed
/// (Section 8.1.1).
#[test]
fn sends_headers_frame_with_transfer_encoding_header() {
use __group::sends_headers_frame_with_transfer_encoding_header as test;
$body
}

/// HTTP/2 does not use the Connection header field (Section 7.6.1 of [HTTP]) to
/// indicate connection-specific header fields; in this protocol,
/// connection-specific metadata is conveyed by other means. An endpoint MUST
/// NOT generate an HTTP/2 message containing connection-specific header fields.
///
/// This includes the Connection header field and those listed as having
/// connection-specific semantics in Section 7.6.1 of [HTTP] (that is,
/// Proxy-Connection, Keep-Alive, Transfer-Encoding, and Upgrade). Any message
/// containing connection-specific header fields MUST be treated as malformed
/// (Section 8.1.1).
#[test]
fn sends_headers_frame_with_upgrade_header() {
use __group::sends_headers_frame_with_upgrade_header as test;
$body
}

/// The only exception to this is the TE header field, which MAY be present in
/// an HTTP/2 request; when it is, it MUST NOT contain any value other than
/// "trailers".
#[test]
fn sends_headers_frame_with_te_trailers() {
use __group::sends_headers_frame_with_te_trailers as test;
$body
}

/// The only exception to this is the TE header field, which MAY be present in
/// an HTTP/2 request; when it is, it MUST NOT contain any value other than
/// "trailers".
#[test]
fn sends_headers_frame_with_te_not_trailers() {
use __group::sends_headers_frame_with_te_not_trailers as test;
$body
}
}
}
}
}
