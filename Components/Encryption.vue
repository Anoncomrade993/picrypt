<script>
  import { fileHandler, updateDetails, pushData, load_image } from './mods/utils.js';

  export default {
    data() {
      return {
        message: '',
        errmsg: '',
      };
    },
    mounted() {
      this.file = this.$refs.file;
      this.src = this.$refs.src;
      this.canvas = this.$refs.canvas;
      this.ctx = this.canvas.getContext('2d');

      fileHandler(this.file, this.src, this.ctx);

      setInterval(() => {
        this.errmsg = '';
      }, 5000);
    },
    methods: {
      async processMessage(e) {
        e.preventDefault();
        if (!this.message || this.message.length === 0) {
          this.errmsg = 'Text box should not be empty';
          return;
        }
        console.log(this.message);
        await pushData(this.message);
      },
      selectFile(e) {
        e.preventDefault();
        this.file.click();
      },
    },
  };
</script>
<template>
  <main>
    <h3>Image encryption</h3>
    <!---
    selector
    src
    btn
    input
    canvas
    --->

    <section class="holder">
      <div class="preview">

        <!----
        input #file
        a #selector
        img #src
        button #btn 
        
        textarea #message 
        canvas #canvas 
        span #err
        ----->
        <input hidden type="file" style="display: none" id="file">
        <p id="link" class="link"><a id="selector" href="javascript:void(0);">select or drop</a></p>
        <img hidden id="src" class="image" alt="source">
      </div>
      <br>
      <br>
      <div class="details">
        <ul>
          <li>image size : </li>
          <li>extension :</li>
          <li>capacity :</li>
          <li>dimension :</li>
        </ul>
      </div>
      <br>
      <br>
      <textarea required autofocus autocomplete="off" placeholder="type here..." id="input" cols="59" rows="0"></textarea>
      <br>
      <br>
      <br>
      <div class="btns">
        <button id="btn" class="btn">encrypt</button>
        <button id="btn" class="btn" style="background-color: #0065be">decrypt</button>
      </div>
      <!--<button id="btn" class="btn">process</button>-->
      <canvas hidden id="canvas" class="canvas"></canvas>
      <br>
      <span id="errmsg"></span>
    </section>
  </main>
</template>

<style scoped>
  main {
    position: relative;
    min-height: 100vh;
    padding: 100px;
    display: block;
    transition: 0.5s;
  }

  main h3 {
    font-size: 40px;
    font-weight: 900;
    text-align: center;
  }

  .holder {
    display: block;
    position: relative;
    padding: 40px;
    height: 100vh;
    border: none;
    background-color: #F2F2F2;
  }

  .holder .preview {
    padding: 6px;
    height: 310px;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden;
  }

  .preview img {
    display: hidden;
    max-width: 100%;
    max-height: 100%;
    margin: 0 auto;
    object-fit: contain;
  }

  .holder .link {
    position: relative;
    margin: 0 auto;
    text-align: center;
    margin-top: 150px;
  }

  .link a {
    color: black;
  }

  .holder .details {
    margin-top: 20px;
  }

  .details ul {
    list-style: none;
    background-color: white;
    padding: 10px;
  }

  .details ul li {
    font-weight: 700;
    font-size: 16px;
  }

  .holder .btn {
    margin-top: 10px;
    position: relative;
    display: block;
    margin: 0 auto;
    width: 20%;
    border: none;
    background-color: black;
    color: white;
    border-radius: 5px;
    font-weight: 500;
    font-size: 20px;
    padding: 6px 15px;
  }

  textarea {
    height: 100px;
    outline: none;
    padding: 10px;
    border-radius: 7px;
  }

  textarea::placeholder {
    color: black;
  }

  #errmsg {
    color: red;
  }

  .btns {
    display: flex;
  }
</style>