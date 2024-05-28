<template>
  <main>
    <h3>Image steganography</h3>
    <section class="holder">
      <div class="preview">
        <input hidden type="file" ref="file" style="display: none">
        <p id="link" class="link"><a href="javascript:void(0);" @click="selectFile">select or drop</a></p>
        <img hidden ref="src" class="image" alt="source">
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
      <textarea required autofocus autocomplete="off" placeholder="type here..." v-model="message" cols="59" rows="3"></textarea>
      <br>
      <br>
      <div class="btns">
        <button @click="processMessage" class="btn">process</button>
      </div>
      <canvas hidden ref="canvas" class="canvas"></canvas>
      <br>
      <span>{{ errmsg }}</span>
    </section>
  </main>
</template>

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