#!/usr/bin/env node

/**
 * Test Echo Sidecar
 *
 * A minimal NDJSON sidecar that reads requests from stdin and writes
 * fake streaming responses to stdout. Used for integration testing
 * the sidecar pipeline without the real Agent SDK.
 *
 * Usage:
 *   echo '{"type":"health_check"}' | node echo.cjs
 *   echo '{"type":"send_message","session_id":1,"content":"hello","model":null,"system_prompt":null}' | node echo.cjs
 */

const readline = require('readline');

const rl = readline.createInterface({ input: process.stdin });

rl.on('line', (line) => {
    try {
        const request = JSON.parse(line);

        if (request.type === 'health_check') {
            console.log(JSON.stringify({ type: 'health_ok', version: '0.1.0' }));
            return;
        }

        if (request.type === 'send_message') {
            // Emit a fake streaming response sequence
            console.log(JSON.stringify({
                type: 'stream_start',
                message_id: 1,
                resolved_model: 'echo-test'
            }));

            const text = `I received your message: "${request.content}". This is a test response from the echo sidecar.`;
            const words = text.split(' ');

            for (let i = 0; i < words.length; i++) {
                console.log(JSON.stringify({
                    type: 'text_delta',
                    content: (i > 0 ? ' ' : '') + words[i]
                }));
            }

            console.log(JSON.stringify({
                type: 'block_complete',
                block_index: 0,
                content_type: 'text'
            }));

            console.log(JSON.stringify({
                type: 'turn_complete',
                input_tokens: 10,
                output_tokens: words.length
            }));
            return;
        }

        if (request.type === 'cancel_stream') {
            console.log(JSON.stringify({ type: 'stream_cancelled' }));
            return;
        }

        if (request.type === 'generate_summary') {
            console.log(JSON.stringify({
                type: 'summary_result',
                session_id: request.session_id,
                summary: `Summary of ${request.messages.length} messages.`
            }));
            return;
        }

        // Unknown request type
        console.log(JSON.stringify({
            type: 'stream_error',
            code: 'unknown_request',
            message: `Unknown request type: ${request.type}`,
            recoverable: false
        }));
    } catch (e) {
        console.log(JSON.stringify({
            type: 'stream_error',
            code: 'parse_error',
            message: e.message,
            recoverable: false
        }));
    }
});
