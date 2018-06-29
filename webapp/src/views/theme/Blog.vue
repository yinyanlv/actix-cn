<template>
    <div id="theme">
        <main>
            <div id="container">
                <div id="mei">
                    <div id="title">
                            <h1> {{ theme.title }} </h1> 
                            <span id="right">
                            <span id="info" class="first"><a :href="'/a/home/' + theme_category_name">{{ theme_category_name }}</a></span> • 
                            <span id="info"><a :href="'/a/user/' + theme_user.user_id">{{ theme_user.username }}</a></span> •   
                            <span id="info">{{ theme_rtime }}</span>  
                            </span>
                    </div>
                </div>
                <div id="body">
                    <div id="theme">
                        <div id="content" v-html="theme.content" ></div>
                    </div>
                    <hr>
                    <div id="comment">
                        <div id="count" style="font-weight: bold; color: #b93bf3;">Comment &nbsp; {{ theme.comment_count }} </div>
                        <div v-for="(comment, index) in theme_comments" :key="index">
                            <div id="detail">
                                <div id="infos">
                                    <span id="info" >{{ index + 1 }} </span>
                                    <span id="info"><a :href="'/a/user/' + comment.user_id">{{ comment.username }}</a></span> • <span id="info">{{ comment.rtime }}</span>
                                </div>
                                <div id="content" v-html="comment.content" > </div>
                            </div>
                        </div>
                    </div>
                    <hr>
                    <div id="reply" v-if="signin_user">
                        <div id="write"> Write comment in markdwon! </div>
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
    name: 'theme',
    data: function() {
        return {
            Comment: '',
            theme: '',
            theme_user: '',
            theme_category_name: '',
            theme_rtime: '',
            theme_comments: '',
            signin_user: ''
        }
    },
    mounted: function() {
        if (sessionStorage.getItem('signin_user')){
            this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
        }
        fetch(URLprefix + 'api/'+ this.$route.params.id,{
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
            this.theme = json.theme
            this.theme_user = json.theme_user
            this.theme_rtime = json.theme_rtime
            if (json.theme_category_name == 'office') json.theme_category_name = '官方'
            if (json.theme_category_name == 'blog') json.theme_category_name = '博客'
            if (json.theme_category_name == 'faq') json.theme_category_name = '问答'
            if (json.theme_category_name == 'share') json.theme_category_name = '分享'
            if (json.theme_category_name == 'job') json.theme_category_name = '招聘'
            this.theme_category_name = json.theme_category_name
            this.theme_comments = json.theme_comments
            console.log(this.theme.content)
        }).catch((e) => {
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
    }
  }
}
</script>

<style scoped>
main {
    padding-bottom: 44px;
}
#body {
    background-color: #ffffff;
}
a {
    color: #0541af;
}
#body #theme > #content {
    margin: 10px;
}
hr {
    height: 11px;
    background-color: #faf5f5;
    border: 0;
}
#body #comment > #count {
    padding: 10px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#body #comment #detail {
    border-bottom: 1px solid rgb(223, 223, 223);
}
#body #comment #detail #infos{
    margin: 10px;
    margin-bottom: 10px;
}
#body #comment #detail #info{
    display: inline-block;
    font-size: 14px;
}
#body #comment #detail #content {
    margin: 10px;
}
#body #reply {
    margin: 10px;
}
#body #reply #write {
    margin-bottom: 10px;
}
#reply textarea {
    width:100%; 
    height: 200px;
}
#body #reply button {
    width:66px; 
    line-height:25px;
    background-color:#ffffff;
    border :1px solid #a39c9c;
}
pre {
    display: block;
    padding: 9.5px;
    margin: 0 0 10px;
    font-size: 14px;
    line-height: 1.42857143;
    word-break: break-all;
    word-wrap: break-word;
    background-color: #f8dff7;
    border: 1px solid #ccc;
    text-shadow: none;
    overflow-x: auto;
  }
  
 code {
    padding: 2px 4px;
    background-color: #f8dff7;
    border-radius: 4px;
    border: 1px solid #ccc;
    text-shadow: none;
  }
  
pre code {
    padding: 0;
    font-size: inherit;
    color: inherit;
    white-space: pre-wrap;
    background-color: transparent;
    border-radius: 0;
    border: 0;
  }
@media only screen and (max-width: 600px) {
    #body  {
      margin: 0.5rem auto;
      width: 95%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    main{
        padding-top: 77px;
    }
    #body  {
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
        background-color: #aaf0db;
    }
   
    #mei #title #right {
        float: right;
        font-size: 14px;
        margin-right: 2rem;
    }
    #mei h1 {
        line-height: 11rem;
        margin: 0 auto;
        padding: 0 4rem;
    }
}
</style>