window.addEventListener('DOMContentLoaded', (e) => {

})


let error = document.getElementById('errmsg');
let details = document.querySelector('.details ul');
let image = document.querySelector('#src');
let link = document.getElementById('link');
let link_2 = document.getElementById('link_2');

let selector = document.getElementById('selector');
let canvas = document.getElementById('canvas')

export const load_image = function(ctx, src, file) {
  try {
    return new Promise(function(res, rej) {
      let image = new Image();
      image.onload = (e) => {
        canvas.width = image.width;
        canvas.height = image.height;

        ctx.drawImage(image, 0, 0, canvas.width, canvas.height);

        let pixels = ctx.getImageData(0, 0, canvas.width, canvas.height);

        // Update the details list
        updateDetails(file, image);

        if (image.complete) {
          selector.style.display = 'none';
          link.style.display = 'none'
          link_2.style.display = 'block';
          document.getElementById('src').style.display = 'block';
        }
        res();
      };

      image.onerror = (e) => {
        err.innerHTML = 'Error loading image';
      };

      image.src = src;
    });
  } catch (e) {
    console.error(e);
  }
}

export const updateDetails = function(file, image) {
  let size = (file.size / 1024).toFixed(2) + ' KB';
  let extension = file.name.split('.').pop();
  let capacity = (file.size * 8 / 1024).toFixed(2) + ' KB';
  let dimensions = `${image.width} x ${image.height}`;

  let detailsHtml = `
    <li>Image size: ${size}</li>
    <li>Extension: ${extension}</li>
    <li>Capacity: ${capacity}</li>
    <li>Dimensions: ${dimensions}</li>
  `;
  details.innerHTML = detailsHtml;
}

export const worker_conn = function(pixels, message ="",action = "") {
  if (!window.Worker) {
    error.innerHTML = 'Browser does not support workers'
  }
  const worker = new Worker('woke.js');

  worker.onmessage = async (event) => {
    console.log(event.data);
  }

  worker.onerror = (err) => error.innerHTML = 'Worker error occurred';

  worker.postMessage({pixels,message,action})
}

setInterval(()=> error.innerHTML = '',4000);