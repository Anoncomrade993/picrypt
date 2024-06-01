
import { load_image, updateDetails } from './mods/utils.js';

let image_src = document.getElementById('src');
let selector = document.querySelector('#selector');
let selector_2 = document.getElementById('link_2');

let btn = document.getElementById('btn');
let dec = document.getElementById('dec');

let input_file = document.getElementById('file');
let canvas = document.getElementById('canvas');
let ctx = canvas.getContext('2d');
let err = document.getElementById('errmsg');

btn.disabled = true;
selector.addEventListener('click', (e) => {
  input_file.click();
});

selector_2.addEventListener('click', (e) => {
  input_file.click();
});


function enableDecryptButton() {
  const decryptButton = document.getElementById('dec');
  decryptButton.disabled = false;
  decryptButton.classList.remove('disabled');
}

// Function to check if an image is loaded
function imageLoaded() {
  const srcImage = document.getElementById('src');
  return srcImage && srcImage.src && srcImage.src !== "";
}

input_file.addEventListener('change', function(ev) {

  let file = ev.target.files[0];
  const reader = new FileReader();

  reader.onload = async function(e) {
    let src = e.target.result;
    await load_image(ctx, src, file);
    image_src.style.display = 'block';

    if (imageLoaded) {
      btn.disabled = false;
      enableDecryptButton();
    }
    image_src.src = src;
  };

  reader.readAsDataURL(file);
});



btn.addEventListener('click', (ev) => {
  //ev.preventDefault()
  let pixels = ctx.getImageData(0,0,canvas.width,canvas.height);
  woker_conn(pixels.data);
});

dec.addEventListener('click', (e) => {
  if (!imageLoaded()) {
    err.innerHTML = 'please pick an image first';
  }
  alert('hello');
});


