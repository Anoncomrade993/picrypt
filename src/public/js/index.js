import { worker_conn, load_image, updateDetails } from './mod/utils.js';



let image_src = document.getElementById('src');
let selector = document.querySelector('#selector');
let selector_2 = document.getElementById('link_2');

let btn = document.getElementById('btn');
let dec = document.getElementById('dec');

let input_file = document.getElementById('file');
let canvas = document.createElement('canvas');
let ctx = canvas.getContext('2d');

let err = document.getElementById('errmsg');

let text_input = document.getElementById('input');



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
  reader.onerror = () => {
    err.innerHTML = 'error reading image';
    return
  }
  reader.readAsDataURL(file);
});




btn.addEventListener('click', async (ev) => {
  try {
    ev.preventDefault();
   
    if (text_input.value.length < 1) {
      err.innerText = 'Minimum length of input is 5';
      return;
    }
    
    document.getElementById('throbber-overlay').classList.remove('throbber-hidden');
    

  } catch (e) {
    console.error(e);
  }
});
dec.addEventListener('click', (ev) => {
  ev.preventDefault();
  alert('hello');
});




async function check_mate(id) {
  try {
    let res = await fetch(`https://jsonplaceholder.typicode.com/users/${id}`);
    if (res.ok) {
      let data = await res.json();
      console.log(JSON.stringify(data, null, 2));
      document.getElementById('throbber-overlay').classList.add('throbber-hidden');
    } else if (res.status == '404') {
      err.innerHTML = '404: User not found'
      setTimeout(() => document.getElementById('throbber-overlay').classList.add('throbber-hidden'), 1000);
      //document.getElementById('throbber-overlay').classList.add('throbber-hidden');
    } else if (res.status == '500') {
      err.innerHTML = 'Server Error Occurred';
      setTimeout(() => document.getElementById('throbber-overlay').classList.add('throbber-hidden'), 1000);
      // document.getElementById('throbber-overlay').classList.add('throbber-hidden');
    }
  } catch (e) {
    throw e
  }
}