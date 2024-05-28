<template>
  <footer>
    <section class="info">
      <h3>More information</h3>
      <ul>
        <li>
          <a href="#">
            <i class="fa fa-life-ring">
              <span style="margin-left: 5px">Tech support</span>
            </i>
          </a>
        </li>
        <li>
          <a href="#">
            <i class="fa fa-info-circle">
              <span style="margin-left: 5px">About</span>
            </i>
          </a>
        </li>
        <li>
          <a href="#">
            <span style="margin-left: 20px">Terms</span>
          </a>
        </li>
      </ul>
    </section>
    <section class="social">
      <h3>Our Handles</h3>
      <ul>
        <li>
          <a href="#">
            <i class="fa fa-github">
              <span style="margin-left: 5px">Github</span>
            </i>
          </a>
        </li>
        <li>
          <a href="#">
            <i class="fa fa-twitter">
              <span style="margin-left: 5px">X</span>
            </i>
          </a>
        </li>
        <li>
          <a href="#">
            <i class="fa fa-google">
              <span style="margin-left: 5px">Gmail</span>
            </i>
          </a>
        </li>
      </ul>
    </section>
  </footer>
  <copyright>
    <span>&copy; <script>document.write(new Date().getFullYear())</script> Picrypt</span>
    <br>
    <span>site designed by <a style="color:white" href="#">Anoncomrade993</a></span>
  </copyright>
</template>

<script>
export default {
  name: 'FooterComponent'
};
</script>

<style scoped>
.cr {

  background-color: black;
  text-align: center;
}

.cr span {
  text-align: center;
  font-size: 16px;
  color: #fff;
}




copyright {
  width: 100%;
  position: relative;
  display: block;
  text-align: center;
  background-color: #000;
  color: #fff;
  font-weight: 300;
  padding: 10px;
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

footer h3 {
  margin-bottom: 3px;
  font-size: 1em;
  font-weight: 500;
}

footer ul > li {
  list-style: none;
}

footer ul li a {
  text-decoration: none;
  color: #fff;
}

@media (max-width: 480px) {
  footer h3 {
    margin-bottom: 3px;
    font-size: 15px;
    font-weight: 800;
  }

  footer ul > li {
    list-style: none;
  }

  footer ul > li > a > i > span,
  footer ul > li > a > span {
    font-size: 12px;
  }
}
</style>