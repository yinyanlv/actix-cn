<template>
  <div id="homecategory">
      <main>
        <div id="container">
          <div id="center">
              <div id="header">
                <li  id="first"><a href="/a/home/best" >最美</a></li>
                <li  ><a href="/a/home/blog" >博客</a></li>
                <li  ><a href="/" >全部</a></li>
                <li  ><a href="/a/home/faq" >问答</a></li>
                <li  ><a href="/a/home/share" >分享</a></li>
                <li  ><a href="/a/home/job" >招聘</a></li>
                <li  ><a href="/a/home/care" >未回复</a></li>
              </div>
              <div id="content">
                      <div id="items" v-for="(theme, index) in theme_list" :key="index">
                            <div id="item">
                                <span id="item-title">
                                  <a :href="'/a/'+ theme.category_name + '/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a>
                                </span>
                                <span id="right">
                                    <span id="info" class="col-name">{{ category_name }}</span>
                                    <span id="info"><a :href="'/a/user/' + theme.user_id">{{ theme.username }}</a></span>
                                    <span id="info"><a :href="'/a/'+ theme.category_name + '/theme/' + theme.id">{{ theme.comment_count }}</a></span>
                                    <span id="info">{{ theme.view_count }}</span>
                                    <span id="info"> {{ theme.rtime }} </span>
                                    <span >  •••  </span>
                                </span> 
                            </div>
                      </div>
              </div>
              <div >
                    <ul id="pagination">
                            <li id="one" ><a :href="'/a/home/' + $route.params.homecategory">1</a></li>
                            <li v-if="page_count > 2"> <a :href="'/a/home/' + $route.params.homecategory + '/2'">2</a></li>

                            <li >••</li>

                            <li v-if="(page_count/2 - 3) > 2" ><a :href="'/a/home/' + $route.params.homecategory + '/' + (page_count/2 - 3)">{{ page_count/2 - 3 }}</a></li>
                            <li v-if="page_count/2 > 2" ><a :href="'/a/home/' + $route.params.homecategory + '/' + page_count/2" >{{ page_count/2 }}</a></li>
                            <li v-if="(page_count/2 + 3) < page_count" ><a :href="'/a/home/' + $route.params.homecategory + '/' + (page_count/2 + 3)" >{{ page_count/2 + 3 }}</a></li>

                            <li >••</li>
                            
                            <li ><a :href="'/a/home/' + $route.params.homecategory + '/' + page_count">{{ page_count }}</a></li>  
                    </ul>       
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
  name: 'homecategory',
  components: {
    "side": Side
  },
  data: function() {
    return {
      theme_list: '',
      signin_user: '',
      page_count: '',
      category_name: ''
    }
  },
  mounted: function() {
       let name = this.$route.params.homecategory
       if (name == 'blog') this.category_name = '博客'
       if (name == 'faq') this.category_name = '问答'
       if (name == 'share') this.category_name = '分享'
       if (name == 'job') this.category_name = '招聘'
       let data = { 
         page_id: 1,
         category_name: this.$route.params.homecategory
       }
       fetch(URLprefix + "api/home/category_list/page_id",{
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.theme_list = json.category_theme_list
                  this.page_count = json.theme_category_page_count
                  console.log(this.theme_list)
              })
              .catch((e) => {
                console.log(e)
              }) 
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
main {
    padding-bottom: 44px;
}
#header {
  background-color: #ffffff;
  box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);
}
#header li {
  display: inline-block;
  line-height: 50px;
  margin-right: 2vw;
  color: #0d8575;
}
#header #first {
  font-weight: bold;
  color: #0d8575;
  margin-left: 0.5vw;
}
#center {
  background-color: #ffffff;
}
#center #content #items #right {
  float: right;
}
#center #items #item {
  padding: 2vh 0.6vw;
  border-bottom: 1px solid #f3e1f8;
}
#center #items #item-title a {
  font-size: 1.1rem;
  color: #0541af;
}
#center #office-title {
  color: #b93bf3;
}
#center #items #right .col-name {
    color: #7a097a;
    font-size: 0.8rem;
}
#center #items #right a {
  color: #0541af;
  font-size: 0.9rem;
}
#center #content #right #info {
  padding-right: 1vw;
}
#center #pagination li {
  display: inline; 
  border: 1px solid #cfd9ee;
  padding: 0.33rem;
  vertical-align: middle;
  line-height: 2.2rem;
}
#center #pagination #one{
  border: 1px solid #5bb383;
  margin-left: 0.4vw;
}
#center #pagination a{
  color: #0541af;
  font-weight: bold;
}
@media only screen and (max-width: 600px) {
    main{
        margin: 0 auto;
        width: 97%;
    }
    #center {
        margin-top: 28px;
    }
    #center #header li {
      margin-left: 2.2vw;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
}
@media only screen and (min-width: 1000px) {
    main {
        margin: 0 auto;
        width: 85%;
        padding: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 1rem;
    }
    #container #side {
        flex: 1;
    }
}
        
</style>