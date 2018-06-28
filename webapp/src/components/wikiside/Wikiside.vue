<template>
  <div  id="wikiside">
        <div id="container">
            <a href="/a/wiki/rust" id="rustnav" title="rust">Rust</a>
            <a href="/a/wiki/actix" id="actixnav" title="actix">Actix</a>
            <a href="/a/wiki/yew" id="yewnav" title="yew">Yew</a>
        </div>
  </div>
</template>

<script>
/* eslint-disable */
export default {
  name: 'wikiside'
}
</script>

<style scoped>
@media only screen and (max-width: 600px) {
    
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
   
}
@media only screen and (min-width: 1000px) {
   #container a {
     display: block;
     padding: 0.6rem 1rem;
   }
}
</style>