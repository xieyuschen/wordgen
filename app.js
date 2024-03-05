import init, { process } from './pkg/wordgen.js';

// Initialize the WASM module
async function initializeWasm() {
    await init();
}
  
initializeWasm().catch(e => console.error("Error initializing WASM:", e));

document.getElementById('upload-btn').addEventListener('click', async () => {
    const fileInput = document.getElementById('upload-file');
    if (fileInput.files.length === 0) {
        alert('Please select a file to process.');
        return;
    }

    const file = fileInput.files[0];
    const reader = new FileReader();

    reader.onload = async (e) => {
        const arrayBuffer = e.target.result;
        const bytes = new Uint8Array(arrayBuffer);
        
        try {
            let decoder = new TextDecoder("utf-8");
            const processedBytes = await process(bytes);
            
            let str = decoder.decode(new Uint8Array(processedBytes))

            const blob = new Blob([str], { type: 'text/html' });
            const url = window.URL.createObjectURL(blob);

            // Create a temporary link to trigger the download
            const a = document.createElement('a');
            a.href = url;
            a.download = "processed_" + file.name+".html";
            document.body.appendChild(a);
            a.click();
            window.URL.revokeObjectURL(url);
            document.body.removeChild(a);
        } catch (error) {
            console.error('Error processing file:', error);
            alert('There was an error processing the file.');
        }
    };

    // Read the file as ArrayBuffer
    reader.readAsArrayBuffer(file);
});