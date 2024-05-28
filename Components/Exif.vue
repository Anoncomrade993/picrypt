<template>
  <main>
    <h3>EXIF Extractor</h3>
    <section class="holder">
      <div class="preview">
        <input hidden type="file" id="file">
        <img hidden id="img">
        <p class="link"><a id="selector" href="javascript:void(0);">select or drop</a></p>
      </div>

      <div class="details">
        <ul>
        </ul>
      </div>
      <br>
      <button id="btn" class="btn">extract</button>
      <br>
      <span id="errmsg">error</span>

    </section>
  </main>
</template>
<style scoped>
  main {
    display: block;
    position: relative;
    margin: 0 auto;
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
    height: 50vh;
    border: none;
    background-color: #F9F9F9;
  }

  .holder .preview {
    padding: 6px;
    height: 310px;
    display: block;
    background-color: #EFEFEF;
  }

  .preview img {
    background-size: cover;
    background-repeat: no-repeat;
    background-position: center;
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
    padding: 10px;
  }

  .details ul li {
    font-weight: 700;
    font-size: 16px;
  }

  .holder .btn {
    margin-top: 10px;
    position: relative;
    padding: 10px 50px;
    display: block;
    margin: 0 auto;
    width: 50%;
    border: none;
    background-color: black;
    color: white;
    border-radius: 5px;
    font-weight: 500;
    font-size: 20px;
  }





  @media (max-width: 768px) {
    main {
      padding: 20px;
    }

    main h3 {
      font-size: 28px;
    }

    .holder .preview {
      height: 150px;
    }

    .holder .link {
      margin-top: 50px;
    }

    .holder .details ul li {
      font-size: 12px;
    }

    .holder .btn {
      width: 70%;
    }
  }

  @media (max-width: 480px) {


    main {
      height: 100vh;
    }

    main h3 {
      font-size: 24px;
    }

    .holder {
      height: 300px;
      padding: 10px;
      background-color: transparent;
    }

    .holder .preview {
      height: 120px;
    }

    .holder .link {
      margin-top: 40px;
    }

    .holder .btn {
      width: 90%;
      font-size: 18px;
    }
  }
</style>