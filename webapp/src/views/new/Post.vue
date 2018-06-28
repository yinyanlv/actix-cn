<template>
    <div id="post">
        <main>
            <div id="container">
                <div id="content">
                    <div id="new-title"><p>New Theme</p></div>
                    <form id="form" >
                            <div id="topic-group">
                                <span  id="category">
                                        <select v-if="username == 'admin'" name="category_name" v-model="CategoryName" id="category-control" >
                                            <option value="muro">muro<span class="icon-arrow"></span></option>
                                            <option value="office">Office</option>
                                            <option v-bind:value="category_name" v-for="(category_name, index) in category_names" :key="index">
                                                    {{category_name}}
                                            </option>
                                        </select>
                                        <select v-else name="category_name" v-model="CategoryName" id="category-control">
                                            <option value="muro">muro<span class="icon-arrow"></span></option>
                                            <option v-bind:value="category_name" v-for="(category_name, index) in category_names" :key="index">
                                                    {{category_name}}
                                            </option>
                                        </select>
                                </span>
                                <span id="title">
                                        <input type="text" name="title" v-model="Title" placeholder="Please input title">
                                </span>
                            </div>    
                            <div id="new">
                                        <textarea name="content" v-model="Content" placeholder="Write new content in markdown!"></textarea>
                            </div>
                            <div id="new">
                                        <button type="submit" id="submit" @click="post" ><span class="tip"> Post </span></button>
                            </div>
                    </form>
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
    name: 'post',
    components: {
        "side": Side
    },
    data () {
        return {
            username: '',
            category_names: '',
            CategoryName: '',
            Title: '',
            Content: ''
        }
    },
    mounted: function() {
        var username = JSON.parse(sessionStorage.getItem('signin_user')).username
        this.username = username
        var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
        let data = { 
            create_user_id: user_id
        }
              fetch(URLprefix + 'api/category_names', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                    this.category_names = json.category_names
              })
              .catch((e) => {
                console.log(e)
              })
    },
    methods: {
        post() {
            var category_name = this.CategoryName
            var title = this.Title
            var content = this.Content
            var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            let data = { 
                user_id: user_id,
                category_name: category_name,
                title: title,
                content: content
            }
            fetch(URLprefix + 'api/theme_new', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                    window.location.reload ( true )
                    this.$router.push('/a/category/' + category_name)
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
#new-title {
    width: 100%;
    line-height: 33px;
    border :1px solid #CAC1C1;
    background-color:#d1f0e3;
}
form #topic-group {
   margin: 11px 0 11px 0;
}
form #topic-group #category #category-control {
    background-color: #FFFFFF;
    border :1px solid #CAC1C1; /*Chrome和Firefox里面的边框是不一样的，所以复写了一下*/
    border: solid 1px #CAC1C1;
    padding-left: 9px;
}
form #topic-group #category #category-control, form #topic-group input {
    height: 30px;
}
form #new textarea {
    width:100%; 
    height: 444px;
}
form #new button {
    width:63px; 
    line-height:25px;
    background-color:rgb(255, 255, 255);
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
    }
    form #topic-group #category #category-control, form #topic-group input {
        width: 100%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
    form #topic-group input {
        width: 72%;
        float: right;
    }
}
@media only screen and (min-width: 1000px) {
  form #topic-group input {
        width: 80%;
        float: right;
  }
  main  {
      margin: 0 auto;
      width: 72%;
      padding-top: 77px;
  }
  #container {
      display: flex;
      flex-flow: row;
    }
    #container #content {
        width: 80%;
        margin-right: 1rem;
    }
    #container #side {
        flex: 1;
    }
}
</style>