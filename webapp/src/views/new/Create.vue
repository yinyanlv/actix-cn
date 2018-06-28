<template>
    <div id="create">
        <!-- <mnav id="mnav"></mnav> -->
        <main>
            <div id="container">
                <div id="center">
                    <div id="content">
                        <div id="top"><p>Crate Community</p></div><br>
                        <div id="topic">
                                <span  id="category">
                                        <select name="community_category" v-model="CommunityCategory" id="category-control" >
                                            <option value="muro">muro<span class="icon-arrow"></span></option>
                                            <option v-bind:value="community_category" v-for="(community_category, index) in community_categorys" :key="index">
                                                    {{community_category}}
                                            </option>
                                        </select>
                                </span>
                                <!-- <span id="title">
                                        <input type="text" name="new_community_name" v-model="NewCommunityNmae" placeholder="Please input a new_community_name">
                                </span> -->
                        </div><br>
                        <input type="text" name="community_name" v-model="CommunityNmae" placeholder="Input community_name"><br><br>
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
            CommunityCategory: '',
            CommunityNmae: '',
            NewCommunityNmae: '',
            community_categorys: ''
        }
    },
    mounted: function() {
        fetch(URLprefix + 'api/community_categorys',{
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
                this.community_categorys = json.community_categorys
        }).catch((e) => {
            console.log(e)
        })
    },
    methods: {
        create () {
            var community_category = this.CommunityCategory
            var community_name = this.CommunityNmae
            var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            let data = { 
                community_category: community_category,
                create_user_id: user_id,
                community_name: community_name
            }
              fetch(URLprefix + 'api/community_new', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                    window.location.reload ( true )
                    this.$router.push("/a/community/"+ community_name)
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
    background-color:#d1f0e3;
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
    background-color: aqua;
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
    }
   #container #side {
        flex: 1;
    }
}
</style>