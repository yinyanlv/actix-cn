<template>
    <div id="theme">
        <main>
            <div id="container">
                <div id="body">
                    <div id="theme">
                        <div id="title">
                            <h2> {{ theme.title }} </h2> 
                            <span id="info"><a :href="'/a/home/' + theme_category_name">{{ theme_category_name_cn }}</a></span> • 
                            <span id="info"><a :href="'/a/user/' + theme_user.id">{{ theme_user.username }}</a></span> •   
                            <span id="info">{{ theme_rtime }}</span>  
                        </div>
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
                <side></side>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import Side from '../../components/side/Side'
export default {
    name: 'theme',
    components: {
        "side": Side
    },
    data: function() {
        return {
            Comment: '',
            theme: '',
            theme_user: '',
            theme_category_name: '',
            theme_category_name_cn: '',
            theme_rtime: '',
            theme_comments: '',
            signin_user: ''
        }
    },
    mounted: function() {
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
            this.theme_category_name_cn = json.theme_category_name_cn
            this.theme_category_name = json.theme_category_name
            this.theme_comments = json.theme_comments
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
              fetch(URLprefix + 'api/theme/' + this.$route.params.id, {
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
#body #theme > #title {
    margin-top: 2px;
    padding: 10px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#body #theme > #title h2 { 
    padding-bottom: 0.3rem;
}
#body #theme > #title #info {
    display: inline-block;
    font-size: 14px;
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
        width: 72%;
        padding-top: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #body {
        width: 80%;
        margin-right: 1rem;
    }
    #container #side {
        flex: 1;
    }
}
</style>