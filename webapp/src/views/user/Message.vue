<template>
    <div id="usermessage">
      <div id="show"><img src="../../../static/imgs/ruster.png" /></div>
      <div id="title">
          <ul>
              <li><a :href="'/a/user/' + $route.params.id" >主题</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/comment'" >评论</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/save'" >收藏</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title">消息</a></li>
          </ul>
      </div>
      <main>
        <div id="container">
            <div id="center">
                <div id="items" v-for="(theme, index) in theme_list" :key="index">
                            <div id="item">
                                <div id="infos">
                                    <span id="info"><a :href="'/a/user/' + theme.user_id">{{ username }}</a></span>
                                    <span id="info"><a :href="'/a/'+ theme.category_name + '/theme/' + theme.id">{{ theme.comment_count }}</a></span>
                                    <span id="info">{{ theme.view_count }}</span>
                                    <span id="info"> {{ theme.created_at }} </span>
                                    <span >  •••  </span>
                                </div> 
                                <div id="item-title">
                                  <a :href="'/a/'+ theme.category_name + '/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a>
                                </div>
                            </div>
                      </div>
            </div>
            <div id="aside">
                <div id="right">
                        <p><strong>{{ username }}</strong></p>
                        <p><strong>{{ email }}</strong></p>
                        <p>created_time : {{ created_time }}</p>

                        <button id="submit" v-if="username == ''" @click="login">Login</button><br/>
                        <button id="submit" v-if="username != ''" @click="update">Update Account</button><br/>
                        <button id="submit" v-if="username != ''" @click="deleteme">Delete Account</button><br/>

                        <div id="update" v-if="userupdate == true">
                            <p>Account Update</p> 
                                <input type="text" name="newname" placeholder="Newname" v-model="Newname"  required /><br/>
                                <input type="text" name="newmail" placeholder="Newmail" v-model="Newmail"  required /><br/>
                                <input type="password" name="newpassword" placeholder="Newpassword" v-model="Newpassword"  required/><br/>
                                <input type="password" name="confirm_newpassword" placeholder="Confirm Newpassword" v-model="ConfirmNewpassword"  required/><br/>
                                <button id="submit" @click="submitnow">UpdateNow</button>
                        </div>
                </div>
            </div>
        </div>
      </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import auth from '../../utils/auth'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'usermessage',
  components: {
    "mnav": Mnav
  },
  data: function() {
    return {
      email: '',
      username: '',
      created_time: '',
      Newname: '',
      Newmail: '',
      Newpassword: '',
      ConfirmNewpassword: '',
      userupdate: false,
      theme_list: ''
    }
  },
  mounted: function() {
      if (sessionStorage.getItem('token')){
        fetch(URLprefix + 'api/user_info',{
            headers: {
                'Authorization': 'Bearer ' + sessionStorage.getItem('token')
            },
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
            this.email =  json.current_user.email
            this.username =  json.current_user.username
            this.created_time =  json.current_user.created_at
        }).catch((e) => {
            console.log(e)
        }) 
      }
      let data = { user_id : Number.parseInt(this.$route.params.id)}
      fetch(URLprefix + 'api/user/id/themes',{
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.theme_list = json.themes
                  console.log(this.theme_list)
              })
              .catch((e) => {
                console.log(e)
              })
  },
  methods: {
    login() {
        window.location.reload ( true ); 
        this.$router.push('/a/signin')
    },
    update() {
        this.userupdate = true
    },
    submitnow() {
        var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
        var newname = this.Newname
        var newmail = this.Newmail
        var newpassword = this.Newpassword
        var confirm_newpassword = this.ConfirmNewpassword
        let data = { 
            user_id: user_id,
            newname: newname,
            newmail: newmail,
            newpassword: newpassword,
            confirm_newpassword: confirm_newpassword
        }
              fetch(URLprefix + 'api/user_update', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                    this.userupdate = false
                    window.location.reload ( true )
              })
              .catch((e) => {
                console.log(e)
              })
    },
    deleteme() {
        fetch(URLprefix + 'api/user_delete',{
            headers: {
                'Authorization': 'Bearer ' + sessionStorage.getItem('token')
            },
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
            sessionStorage.removeItem('token')
            sessionStorage.removeItem('signin_user')
            window.location.reload ( true ); 
            this.$router.push('/')
        }).catch((e) => {
            console.log(e)
        })   
    }
  }
}
</script>

<style scoped>
#show {
    background-color: #f1a3d6;
}
#title {
    line-height: 3.3rem;
    background-color: #faeaf5;
}
#title #message-title {
    padding-bottom: 0.2rem;
    border-bottom: 3px solid #a506a5;
}
#container a{
    color: #0541af;
}
#center {
    background-color: #ffffff;
}
#center #items #item {
    padding: 1.2vh 0.5vw;
    border-bottom: 1px solid #f3e1f8;
}
#center #items #item-title {
    margin-top: 1vh;
    font-size: 1.1rem;
}
#center #items #infos {
    font-size: 0.9rem;
}
button {
    width: 7rem; 
    line-height:25px;
    background-color:#ffffff;
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
  img {
      margin: 0.3rem 1rem 0.1rem;
      width: 5rem;
      height: 5rem;
  }
  #title ul li {
      display: inline-block;
      padding-left: 1rem;
      font-weight: bold;
  }
  #center  {
      margin: 1rem;
      width: 95%;
  }
  #center #container #right {
      margin: 1rem auto;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
  #show {
      padding-top: 6rem;
  }
  img {
      margin-left: 10vw;
      width: 8rem;
      height: 8rem;
  }
  #title ul {
      margin-left: 10vw;
  }
  #title ul li {
      display: inline-block;
      font-weight: bold;
  }
  #title ul #item {
      padding-left: 2rem;
  }
  #center  {
      margin: 0 auto;
      width: 80%;
      padding-top: 2rem;
  }
  #center #container {
    display: flex;
    flex-flow: row;
  }
  #center #container #left {
      width: 80%;
      padding-right: 1rem;
  }
  #center #container #right {
      flex: 1;
  }
  #center #container #left #update {
    margin: 2rem auto;
  }
}
@media only screen and (min-width: 1000px) {
    #show {
        padding-top: 6rem;
    }
    img {
        margin-left: 13vw;
        width: 8rem;
        height: 8rem;
        border-radius: 50%;
    }
    #title ul {
        margin-left: 12vw;
    }
    #title ul li{
        display: inline-block;
        font-weight: bold;
        padding-left: 2rem;
    }
   main {
        margin: 1rem auto;
        padding-bottom: 1rem;
        width: 72%;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 1rem;
    }
    #container #aside {
        flex: 1;
    }

    #container #aside #right{
        padding: 1rem;
        border: 1px solid rgb(212, 212, 212);
        background-color: #ffffff;
    }
    #container #aside #right #update {
        margin: 2rem auto;
    }
}
</style>