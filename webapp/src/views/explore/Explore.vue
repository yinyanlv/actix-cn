<template>
  <div  id="explore">
      <!-- <mnav id="mnav"></mnav> -->
      <main>
        <div id="container">
          <div id="content">
              
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
  name: 'explore',
  components: {
    "side": Side
  },
  data: function() {
    return {
      communitys:'',
      community_category_list: ''
    }
  },
  mounted: function() {
     
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
main {
    padding-bottom: 44px;
}
#content #community  #title {
    border-bottom: 1px solid rgb(223, 223, 223);
}
#content #community #community_category {
   font-weight: bold;
    margin: 10px 0;
    background-color: aquamarine;
}
#content #community #detail {
    margin: 10px 0 10px 33px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#content #community #detail #community_name {
    font-weight: bold;
    color: #261cb6;
}
#content #community #detail #infos{
    margin: 10px 0;
    margin-bottom: 10px;
}
#content #community #detail #info{
    display: inline-block;
    font-size: 14px;
}
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
      margin: 0 auto;
      width: 72%;
      padding-top: 77px 0 22px 0;
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
    #container #content {
        width: 80%;
        margin-right: 1rem;
    }
    #container #side {
        flex: 1;
    }
}
</style>