<template>
    <div id="blog">
        <main>
            <div id="container">
                <div id="mei">
                    <div id="title">
                            <h1> {{ theme.title }} </h1> 
                            <span v-if="saveorno == 'false'" id="save" @click="save">收藏</span>
                            <span v-else id="saved">已收藏</span>
                            <span id="like">喜欢 <span id="likeid">{{like}} </span> </span>
                            <span id="right"> 
                            <span id="info" class="first"><a :href="'/a/home/' + theme_category_name">博客</a></span> • 
                            <span id="info"><a :href="'/a/user/' + theme_user.id">{{ theme_user.username }}</a></span> •   
                            <span id="info">{{ theme_rtime }}</span>  
                            </span>
                    </div>
                </div>
                <div id="center">
                    <div id="theme">
                        <div id="content" v-html="theme.content" ></div>
                    </div>
                    <hr>
                    <div id="comment">
                        <div id="count">Comment &nbsp; {{ theme.comment_count }} </div>
                        <div v-for="(comment, index) in theme_comments" :key="index">
                            <div id="detail">
                                <div id="infos">
                                    <span id="info" >{{ index + 1 }}&nbsp;</span>
                                    <span id="info"><a :href="'/a/user/' + comment.user_id">{{ comment.username }}</a></span> • <span id="info">{{ comment.rtime }}</span>
                                </div>
                                <div id="content" v-html="comment.content" > </div>
                            </div>
                        </div>
                    </div>
                    <hr>
                    <div id="reply" v-if="signin_user">
                        <div id="write"> 先<a href="https://maxiang.io/" target="view_window">在线MD编辑/预览</a>，再复制过来</div>
                        <textarea name="comment" v-model="Comment" placeholder="if you want @somebody for send a message in your comment, the rule is: 
                        1: the @ symbol can't be first position at line.(like: @somebodyxxxxx)
                        2: one position before the @ symbol can't be space(like: xxxxx @somebodyxxxxx)."></textarea><br>
                        <button id="submit" @click="comment">Comment</button>
                    </div>  
                    <div v-else style="margin: 10px;">Please login first and make a Comment.
                        <a href="/a/access" style="background-color:aqua;">Login</a>
                    </div>    
                </div>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
export default {
    name: 'blog',
    data: function() {
        return {
            Comment: '',
            theme: '',
            theme_user: '',
            theme_category_name: '',
            theme_rtime: '',
            theme_comments: '',
            signin_user: '',
            like: '',
            saveorno: ''
        }
    },
    mounted: function() {
        this.saveorno = 'false'
        if (sessionStorage.getItem('signin_user')){
            this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
        }
        fetch(URLprefix + 'api/theme/'+ this.$route.params.id,{
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
            this.theme = json.theme
            this.theme_user = json.theme_user
            this.theme_rtime = json.theme_rtime
            this.theme_category_name = json.theme_category_name
            this.theme_comments = json.theme_comments
        }).catch((e) => {
            console.log(e)
        })
        let theme_id = this.$route.params.id
        var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
        let data = { 
            theme_id: Number.parseInt(theme_id),
            user_id: user_id
        }
        fetch(URLprefix + 'api/blog/like', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.like = json.number
                  this.saveorno = json.saveorno
                  console.log(this.saveorno)
              })
              .catch((e) => {
                console.log(e)
              })
  },
  methods: {
    comment () {
        let comment = this.Comment
        let theme_id = this.$route.params.id
        let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
        let data = {
            the_theme_id: theme_id,
            user_id: user_id,
            comment: comment
        }
        fetch(URLprefix + 'api/' + this.$route.params.id, {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                  console.log(json)
                  window.location.reload ( true )
              })
              .catch((e) => {
                console.log(e)
              })        
    },
    save(){
            let save = document.getElementById("save")
            let likeid = document.getElementById("likeid")
            save.style.color = "green"
            save.innerHTML = "已收藏"
            likeid.innerHTML = Number.parseInt(likeid.innerHTML) + 1
            let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            let theme_id = this.$route.params.id
            let data = { 
                user_id: Number.parseInt(user_id),
                theme_id: Number.parseInt(theme_id)
            }
            fetch(URLprefix + 'api/blog/save', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  console.log(json)
              })
              .catch((e) => {
                console.log(e)
              })
    }
  }
}
</script>

<style scoped>
main {
    padding-bottom: 44px;
}
#center {
    background-color: #ffffff;
}
a {
    color: #0541af;
}
#body #comment,#body #reply {
    border-top: 1px solid fuchsia;
}
#center #theme > #content {
    margin: 10px;
}
hr {
    height: 11px;
    background-color: #faf5f5;
    border: 0;
}
#center #comment > #count {
    font-weight: bold;
    color: fuchsia;
    padding: 10px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#center #comment #detail {
    border-bottom: 1px solid rgb(223, 223, 223);
}
#center #comment #detail #infos{
    margin: 10px;
    margin-bottom: 10px;
}
#center #comment #detail #info{
    display: inline-block;
    font-size: 14px;
}
#center #comment #detail #content {
    margin: 10px;
}
#center #reply {
    padding: 10px;
}
#center #reply #write {
    margin-bottom: 10px;
}
#reply textarea {
    width:100%; 
    height: 200px;
    border: 0.2px solid #d4d2d3;
}
#center #reply button {
    width:66px; 
    line-height:25px;
    background-color:#ffffff;
    border :1px solid #a39c9c;
}

@media only screen and (max-width: 600px) {
    #center  {
      margin: 0.5rem auto;
      width: 95%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    main{
        padding-top: 77px;
    }
    #center  {
        margin: 0 auto;
        width: 72%;
  }
}
@media only screen and (min-width: 1000px) {
    main {
        margin: 0 auto;
        width: 66%;
        padding-top: 77px;
    }
    #mei {
        margin: -1vh 0 1vh;
        height: 13rem;
        /* background-color: #40e0d0; */
        background-color: #59C173;
    }
    #mei h1 {
        line-height: 11rem;
        margin: 0 auto;
        padding: 0 4rem;
    }
    #mei #title #save, #mei #title #saved {
        font-size: 15px;
        margin-left: 4.1rem;
        padding: 0.8vh 0.3vw 0.3vh 0.2vw;
    }
    #mei #title #save {
        border: 0.1px solid fuchsia;
        color: fuchsia;
    }
    #mei #title #saved {
        color: green;
    }
    #mei #title #like {
        font-size: 15px;
        margin-left: 2rem;
        color: fuchsia;
    }
    #mei #title #right {
        float: right;
        font-size: 14px;
        margin-right: 2rem;
    }
    
}
</style>