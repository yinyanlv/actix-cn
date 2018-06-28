<template>
    <div id="side">
        <div id="one">
            <h3>What</h3>
            <p>Chania is a city on the island of Crete.</p>
            <h3>Where</h3>
            <p>Crete is a Greek island in the Mediterranean Sea.</p>
            <h3>How</h3>
            <p>You can reach Chania airport from all over Europe.</p>
            </div>
    </div>
</template>

<script>
/* eslint-disable */
export default {
  name: 'side'
}
</script>

<style scoped>
@media only screen and (max-width: 600px) {
    
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
   
}
@media only screen and (min-width: 1000px) {
   #one{
        padding: 1rem;
        border: 1px solid rgb(212, 212, 212);
        background-color: #ffffff;
    }
}
</style>