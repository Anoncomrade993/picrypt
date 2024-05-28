<script>

</script>
<template>
  <main>
    <h3>Media download </h3>
    <section class="holder">
      <div class="preview">
        <img hidden class="image" src="./assets/12065_2021_683_Fig1_HTML.png" alt="source">
        <canvas hidden class="canvas"></canvas>
      </div>
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
      <button class="btn"><i aria-hidden="true" class="fa fa-download"></i> <span style="margin-left:10px">download</span></button>
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



  header {
    position: relative;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: thin ridge #ccc;
    width: 100%;
    background-color: #fff;
    padding: 10px;
  }

  header h2 {
    font-size: 4em;
    font-weight: 900;
    color: var(--main);
  }

  header nav {
    position: relative;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  header nav ul {
    display: flex;
    position: relative;
  }

  header nav ul li {
    list-style: none;
  }

  header nav ul li a {
    text-decoration: none;
    display: inline-block;
    position: relative;
    margin-left: 40px;
    font-weight: 400;
    font-size: 30px;
    color: #000;
    background-color: transparent;
    padding: 10px 15px;

  }

  header .fa {
    cursor: pointer;
    border-radius: 100%;
    color: #000;
    font-size: 50px;
    border: none;
  }


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
  }

  .holder .preview {
    padding: 6px;
    height: 310px;
    display: block;
    border: 2px dashed #ccc;
    background-color: #F4F4F4;
  }

  .preview img {
    background-size: cover;
    background-repeat: no-repeat;
    background-position: center;
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
    margin-top: 50px;
    position: relative;
    padding: 10px 50px;
    display: block;
    margin: 0 auto;
    width: 50%;
    border: none;
    background-color: black;
    color: white;
    border-radius: 5px;
    font-weight: 100;
    font-size: 20px;
  }


  footer {
    width: 100%;
    position: relative;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 50px;
    background-color: #000;
    color: #fff;
  }


  footer h2 {
    margin-bottom: 3px;
    font-size: 1em;
    font-weight: 500;
  }

  footer ul>li {
    list-style: none;
  }

  footer ul li a {
    text-decoration: none;
    color: #fff;

  }

  @media (max-width: 768px) {
    main {
      padding: 20px;
    }

    header h2 {
      font-size: 2em;
    }

    header nav ul li a {
      font-size: 20px;
      margin-left: 10px;
    }

    header .fa {
      font-size: 25px;
    }

    main h3 {
      font-size: 28px;
    }

    .holder .preview {
      height: 150px;
    }



    .holder .btn {
      width: 70%;
    }
  }

  @media (max-width: 480px) {

    footer h3 {
      margin-bottom: 3px;
      font-size: 15px;
      font-weight: 800;
    }

    footer ul>li {
      list-style: none;
    }

    footer ul>li>a>i>span,
    footer ul>li>a>span {
      font-size: 12px;
    }

    header {
      padding: 5px;
    }

    header h2 {
      font-size: 1.5em;
      margin-bottom: 10px;
    }

    textarea {
      width: 300px;
      height: 50px;
    }

    header nav ul li a {
      font-size: 18px;
      margin-left: 0;
      margin-bottom: 10px;
    }

    header .fa {
      font-size: 20px;
    }

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



    .holder .btn {
      width: 90%;
      font-size: 18px;
    }
  }
</style>