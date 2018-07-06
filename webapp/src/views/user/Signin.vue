<template>
  <div id="signin">
      <!-- <mnav id="mnav"></mnav> -->
      <div id="content">
          <div id="title">    
            <router-link to="/a/signin">Sign In &emsp;&emsp;|&emsp;&emsp;</router-link>
            <router-link to="/a/signup">Sign Up</router-link> 
          </div>
            <input type="text" name="username" placeholder="Username" v-model="Username" />
            <input type="password" name="password" placeholder="Password" v-model="Password" /><br/>
          <div id="add">
            <label class="checkbox">
              <input type="checkbox" name="remember" value="1" id="checks" checked="true"/>
              <span class="check" for="checks"></span> 
            </label>Remember
            <span> &emsp;&emsp;<router-link to="/settings/missing_pwd" style="text-decoration-line: none;">Forgot Password?</router-link></span><br/>
          </div>
          <div>
              <div id="v_container" style="height: 44px;"></div>
              <input type="text" id="code_input" value="" placeholder="请输入验证码" style="width: 80%;"/>
              <span><button id="my_button" style="width: 20%; padding: 6px 0;background-color: bisque;">验证</button></span>
          </div>
          <button id="submit" @click="signin">Sign in</button><br/>
          <div id="text"> Login with social </div>
          <button class="social-signin facebook">facebook</button>
          <button class="social-signin twitter">Twitter</button>
          <button class="social-signin google">Google+</button>
      </div>
  </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import  '../../../static/js/gVerify.js'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'signin',
  components: {
    "mnav": Mnav
  },
  data () {
    return {
      Username: '',
      Password: ''
    }
  },
  mounted: function() {
    var verifyCode = new GVerify("v_container");
    document.getElementById("my_button").onclick = function () {
      var res = verifyCode.validate(document.getElementById("code_input").value);
      if (res) {
        let verify = document.getElementById("my_button")
        verify.innerHTML = "成功"
      } else {
        let verify = document.getElementById("my_button")
        verify.innerHTML = "失败"
      }
    }
  },
  methods: {
    signin () {
      var username = this.Username
      var password = this.Password
      let data = { 
          username: username,
          password: password
      }
      if (document.getElementById("my_button").innerHTML == "成功") {
            fetch(URLprefix + 'user/signin', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                    if (json.status == 200) {
                        sessionStorage.setItem('token',json.token);
                        sessionStorage.setItem('signin_user',JSON.stringify(json.signin_user));
                        window.location.reload ( true ); 
                        this.$router.push('/')
                    }else{
                        alert(json.message)
                    }
              })
              .catch((e) => {
                console.log(e)
              })
      }else{
          alert("请先成功通过验证码再登陆.")
      }
              
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#content {
    width: 250px;
    margin: 0 auto;
    padding-top: 33px;
}
#title {
    padding: 0.5rem 4px;
    font-size: 22px;
    font-weight: bold;
    background-color:bisque;
}
input[type="text"],
input[type="password"] {
  margin: 6px auto auto;
  width: 250px;
  height: 36px;
  border: none;
  border-bottom: 1px solid #AAA;
  font-size: 16px;
}
#add {
  margin: 10px 0;
}
#submit  {
  margin: 10px 0;
  width: 250px;
  height: 33px;
  background-color:bisque;
  border: none;
  border-radius: 2px;
  font-weight: bold;
}
#text {
  width: 250px;
  border-radius: 3px;
  padding: 0.3rem 0.8em;
  background-color:bisque;
  border-bottom: 1px solid #f1b75c;
  font-weight: bold;
  text-align: center;
}
button.social-signin {
  margin: 10px 0;
  width: 83.3px;
  height: 33px;
  border: none;
  border-radius: 2px;
  color: #FFF;
}
button.social-signin:hover,
button.social-signin:focus {
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.4);
  transition: 0.2s ease;
}
button.social-signin:active {
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.4);
  transition: 0.2s ease;
}
button.social-signin.facebook {
  background: #32508E;
}
button.social-signin.twitter {
  background: #55ACEE;
}
button.social-signin.google {
  background: #DD4B39;
}
@media only screen and (min-width: 600px) {
    #content {
      margin: 0 auto;
      padding-top: 100px;
    }
}
</style>