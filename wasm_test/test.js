const raptorq = require('raptorq');

function runTest() {
    console.log("Running raptorq WASM test...");

    // 1. Create some data to encode
    const data = new Uint8Array(1024 * 512); // 512 KB of data
    for (let i = 0; i < data.length; i++) {
        data[i] = i % 256;
    }
    console.log("Created data of size: " + data.length);

    // 2. Create an encoder
    const mtu = 1280; // Maximum Transmission Unit
    const encoder = new raptorq.Encoder(data, mtu);
    console.log("Encoder created.");

    // 3. Get encoded packets
    const repairPacketsPerBlock = 5;
    const packets = encoder.get_encoded_packets(repairPacketsPerBlock);
    console.log("Generated " + packets.length + " encoded packets.");

    // 4. Create a decoder
    const decoder = new raptorq.Decoder(data.length, mtu);
    console.log("Decoder created.");

    // 5. Decode the packets
    let decodedData = null;
    for (const packet of packets) {
        decodedData = decoder.decode(packet);
        if (decodedData) {
            break;
        }
    }

    // 6. Verify the result
    if (decodedData) {
        console.log("Decoding successful. Verifying data...");
        if (decodedData.length !== data.length) {
            console.error("Test failed: Decoded data has a different length.");
            return;
        }
        let match = true;
        for (let i = 0; i < data.length; i++) {
            if (data[i] !== decodedData[i]) {
                match = false;
                break;
            }
        }
        if (match) {
            console.log("Test passed: Decoded data matches original data.");
        } else {
            console.error("Test failed: Decoded data does not match original data.");
        }
    } else {
        console.error("Test failed: Decoding did not complete.");
    }
}

try {
    runTest();
} catch (e) {
    console.error("An error occurred during the test:", e);
}