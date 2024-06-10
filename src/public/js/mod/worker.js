import { run_decryption, run_encryption } from '../loader.js';


// Listen for messages from the main script
self.onmessage = async function(event) {
  let { pixels, message, action } = event.data;
  console.log(pixels.length, message, action);
  run_encryption(pixels, message);
  //await post_req(pixels, message, action);
};

self.onerror = function(err) {
  postMessage({ error: err });
};



const post_req = async function(pixels, message = "", action = "") {

  return new Promise(async (resolve, reject) => {
    try {
      
      switch (action) {
        case 'enc':
          run_encryption(pixels, message);
          break;
        case 'dec':
          //res = await fetch('API_URL', { method: "POST", body: JSON.stringify({ image_data: pixels }) });
          break;
        default:
          break;
      }

    } catch (e) {
      console.log(e);
    }

  });
};