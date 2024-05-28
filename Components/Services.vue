<template>
  <section class="services">
    <h2>Our Services</h2>
    <section class="gridHolder">
      <div>
        <h3>Image Steganography</h3>
        <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit. Veritatis consectetur aut alias officiis eum, facere quam animi accusantium dolor totam sapiente non culpa vitae possimus explicabo, asperiores voluptate quibusdam labore?</p>
        <a class="r" href="#">Try it</a>
      </div>
      <div>
        <h3>Image Encryption</h3>
        <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit. Eveniet quia perferendis vel magnam, esse consequuntur doloribus dolorum quis, facere animi repudiandae asperiores sapiente. Magni nesciunt mollitia molestias delectus quisquam, labore!</p>
        <a class="r" href="#">Try it</a>
      </div>
      <div>
        <h3>EXIF Extractor</h3>
        <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit. Aliquid quidem cumque neque, mollitia delectus, a saepe placeat totam dignissimos adipisci soluta id recusandae aperiam perspiciatis laudantium numquam iusto quisquam accusamus.</p>
        <a href="#" class="r">Try it</a>
      </div>

      <div>
        <h3>Image-Image Encryption</h3>
        <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit. Ipsam excepturi voluptatem quas! Fuga quidem perspiciatis ipsam enim aspernatur doloremque, fugit qui harum culpa rerum animi vero quod at cupiditate inventore.</p>
        <a class="r na" href="#">Not available</a>
      </div>
      <div>
        <h3>Audio Steganography</h3>
        <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit. Voluptatibus rerum expedita blanditiis officiis, nisi, accusantium molestias, soluta fugit omnis, iste maxime. Obcaecati in vel iste reiciendis eligendi possimus veritatis expedita.</p>
        <a class="r na" href="#">Not available</a>
      </div>
      <div>
        <h3>Audio Encryption</h3>
        <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit. Nobis maiores, debitis natus repellendus minima, quas sequi accusantium optio eligendi ducimus consequuntur excepturi corporis, quidem tempora quae. Quasi soluta expedita exercitationem.</p>
        <a class="r na" href="#">Not available</a>
      </div>
    </section>
    <br>
    <section class="faq">
      <h2>FAQs</h2>
      <div>
        <h3>What is Steganography</h3>
        <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit. Mollitia saepe sequi, ad ipsam quod? Dolorum nostrum laborum omnis, maxime iusto corrupti harum laboriosam voluptatibus modi adipisci, nobis recusandae, earum suscipit!</p>
      </div>
      <div>
        <h3>What is encryption</h3>
        <p>Inventore, facilis impedit dolor, excepturi, quidem aliquam eaque magni exercitationem vero id perspiciatis? Tempora sed cum quo at ut perferendis, aperiam molestias inventore sint nam possimus. Sequi atque vero laboriosam.</p>
      </div>
      <div>
        <h3>What are pixels</h3>
        <p>Laborum, inventore veritatis rerum ea quidem culpa mollitia nesciunt dolor distinctio, eveniet velit, doloribus, dolores amet eius quisquam maxime alias deserunt? Delectus quibusdam nesciunt molestiae quisquam harum distinctio, est saepe!</p>
      </div>
      <div>
        <h3>What's the essence of encryption </h3>
        <p>Distinctio, ullam tempora dignissimos neque corporis voluptates, corrupti ratione voluptate vel dolor unde asperiores aperiam vitae! Consectetur ipsam voluptates natus voluptate quae reiciendis soluta totam minus, beatae facere. In, labore.</p>
      </div>
    </section>
  </section>
</template>
<style scoped>
  a.r {
    margin-top: 10px;
    display: inline-block;
    padding: 10px 50px;
    text-decoration: none;
    color: #fff;
    text-align: center;
    border-radius: 15px;
    background-color: #7AD649;
  }

  .na {
    pointer-events: none;
    cursor: default;
    opacity: 0.5;
  }


  .services {
    display: hidden;
    margin-top: 5vh;
  }

  .services h2 {
    font-size: 2em;
    font-weight: 800;
    color: #bbb;
  }


  .gridHolder {
    display: block;
    margin: 0 auto;
  }

  .gridHolder>div,
  .faq>div {
    padding: 20px;
    border: thin ridge #ccc;
    height: 200px;
    border-radius: 10px;
    margin-top: 15px;
  }

  .faq>div {
    border: none;
  }

  .faq>div>h3,
  .gridHolder>div>h3 {
    margin-top: 10px;
  }


  /* For screens smaller than 768px */
  @media (max-width: 768px) {


    .services h2 {
      font-size: 1.5em;
    }

    .services,
    .faq {
      margin-top: 5vh;
    }

    .services h2,
    .faq h2 {
      font-size: 1.5em;
    }
  }

  /* For screens smaller than 480px */
  @media (max-width: 480px) {

    .services h2 {
      font-size: 1.2em;
    }

    .services,
    .faq {
      margin-top: 3vh;
    }

    .services h2,
    .faq h2 {
      font-size: 1.2em;
    }

    .services section {
      padding: 10px;
    }

    .services {
      margin-bottom: 50px;
    }

    .faq {
      padding-top: 50px;
    }

    .services section>div,
    .faq section>div,
    {
    height: auto;
  }

  }
  
</style>