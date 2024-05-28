<template>
  <header>
    <h2>Picrypt</h2>
    <!-- <a href="#"><img src="./assets/secured.svg" class="logo" alt="logo"></a> -->
    <nav>
      <ul>
        <li><a id="git" href="#"><i class="fa fa-github"></i></a></li>
        <li><a href="#"><i class="fa fa-twitter"></i></a></li>
      </ul>
    </nav>
  </header>
  <div class="tab">
    <ul>
      <li><a href="#">Steganograpghy</a></li>
      <li><a href="#">Encryption</a></li>
      <li><a href="#">Exif</a></li>
      <li><a href="#">Decryption</a></li>
    </ul>
  </div>
</template>

<script>
export default {
  name: 'HeaderComponent',
};
</script>

<style scoped>
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

.tab {
  margin-top: 20px;
}

.tab ul {
  display: flex;
  justify-content: center;
  list-style: none;
  padding: 0;
}

.tab ul li {
  margin-right: 20px;
}

.tab ul li a {
  text-decoration: none;
  font-size: 20px;
  color: #000;
  padding: 10px 15px;
  background-color: transparent;
}

@media (max-width: 768px) {
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
}

@media (max-width: 480px) {
  header {
    padding: 5px;
  }

  header h2 {
    font-size: 1.5em;
    margin-bottom: 10px;
  }

  header nav ul li a {
    font-size: 18px;
    margin-left: 0;
    margin-bottom: 10px;
  }

  header .fa {
    font-size: 20px;
  }

  .tab ul li {
    margin-right: 10px;
  }

  .tab ul li a {
    font-size: 18px;
  }
}
</style>