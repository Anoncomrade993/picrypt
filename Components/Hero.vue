<template>
  <section class="hero">
    <div class="content">
      <h2>It's more than,<br><span>Just pixels.</span></h2>
      <div class="sample">
        <a href="https://media.springernature.com/lw685/springer-static/image/art%3A10.1007%2Fs12065-021-00683-x/MediaObjects/12065_2021_683_Fig1_HTML.png"><img src="/assets/12065_2021_683_Fig1_HTML.png" alt="steganography illustration"></a>
      </div>
      <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit. Impedit numquam mollitia consequuntur nam, veniam? Illum ex eius, error nobis dicta esse ratione non atque assumenda consequatur doloribus vero, repudiandae aut!</p>
      <br>
      <br>
    </div>
  </section>
</template>

<style scoped>
  hero {

  position: relative;
  width: 100%;
  display: block;
  justify-content: space-between;
  align-items: center;
}

.hero .content h2 {
  font-weight: 400;
  font-size: 4em;
  line-height: 1.5em;
  color: #000;
}

.hero .content .button {
  cursor: pointer;
  background-color: #F66919;
  border-radius: 20px;
  text-decoration: none;
  margin-top: 15px;
  outline: none;
  padding: 13px 50px;
  color: #fff;
  font-weight: 600;
  border: none;
  font-size: 25px;
}

.hero p {
  margin-top: 20px;
  font-size: 25px;
  font-weight: 400;
  color: #000;
}

.hero .content h2>span {
  font-weight: 900;
}

.hero .content .sample {
  margin-top: 30px;
  border-bottom: thin ridge #ccc;
}

.hero .content .sample a {
  color: #0065be;
  text-decoration: none;
}

.hero .content .sample img {
  width: 100%;
  background-size: cover;
  background-repeat: no-repeat;
}

@media (max-width: 768px) {
  
  .hero .content h2 {
    font-size: 3em;
  }

  .hero .content a {
    padding: 8px 30px;
    font-size: 16px;
  }

  .hero p {
    font-size: 15px;
  }


}

/* For screens smaller than 480px */
@media (max-width: 480px) {

  
  .hero .content h2 {
    font-size: 2.5em;
  }


  .hero .content .button {
    padding: 10px 20px;
    font-size: 14px;
    font-weight: 400;

  }

  .hero p {
    font-size: 14px;
  }

}

</style>