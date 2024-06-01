// Listen for messages from the main script
onmessage = function(event) {
  let data = event.data;
//  console.log(pixels.length, message, action);
  /*let res = await post_req(pixels, message, action);
  if (res) {}
  */
  postMessage(data);

};
onerror = function(err) {
  postMessage({ error: err.message });
};

const post_req = async function(pixels, message = "", action = "") {
  return new Promise(async (resolve, reject) => {
    try {
      let res;
      switch (action) {
        case 'enc':
          if (message.length < 5) {
            error.innerHTML = 'inputbox minimum length is 5';
          }
          res = await fetch('API_URL', { method: "POST", body: JSON.stringify({ image_data: pixels, input_data: message }) });
          break;
        case 'dec':
          res = await fetch('API_URL', { method: "POST", body: JSON.stringify({ image_data: pixels }) });
          break;
        default:
          break;
      }
      if (!res.ok) {
        reject();
      }
      let data = await res.json();
      let { pixels } = data;
      resolve(pixels);
    } catch (e) {
      reject(e);
    }
  });
}