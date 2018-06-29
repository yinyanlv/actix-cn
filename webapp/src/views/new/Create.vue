<template>
    <div id="create">
        <!-- <mnav id="mnav"></mnav> -->
        <main>
            <div id="container">
                <div id="center">
                    <div id="content">
                        <div id="top"><p>Crate Category</p></div><br>
                        <input type="text" name="category_name_cn" v-model="CategoryNmaeCN" placeholder="中文表示"><br><br>
                        <input type="text" name="category_name" v-model="CategoryNmae" placeholder="英文表示"><br><br>
                        <button type="submit" id="submit" @click="create" ><span class="tip"> Create </span></button>
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
    name: 'create',
    components: {
        "side": Side
    },
    data () {
        return {
            CategoryNmae: '',
            CategoryNmaeCN: ''
        }
    },
    methods: {
        create () {
            let category_name = this.CategoryNmae
            let category_name_cn = this.CategoryNmaeCN
            let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            let data = { 
                user_id: Number.parseInt(user_id),
                category_name: category_name,
                category_name_cn: category_name_cn
            }
              fetch(URLprefix + 'api/category_new', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                    window.location.reload ( true )
                    this.$router.push("/")
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
#top {
    line-height: 33px;
    background-color:#f5fdfa;
    border :1px solid #CAC1C1;
}

#topic #category #category-control {
    background-color: #FFFFFF;
    border :1px solid #CAC1C1; /*Chrome和Firefox里面的边框是不一样的，所以复写了一下*/
    border: solid 1px #CAC1C1;
    text-align: center;
}
#topic #category #category-control, #topic input {
    height: 30px;
}
#content input {
    line-height: 30px;
}
button {
    width:63px; 
    line-height:30px;
    background-color: #FFFFFF;
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
    }
    #topic #category #category-control, #topic input {
        width: 100%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
    #topic input {
        width: 72%;
        float: right;
    }
}
@media only screen and (min-width: 1000px) {
  main  {
      margin: 0 auto;
      width: 72%;
      padding-top: 77px;
  }
  #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 1rem;
        background-color: #FFFFFF;
    }
   #container #side {
        flex: 1;
    }
}
</style>