<script>
export default {
  data() {
    return {
      count: 0,
      titleClass: 'title'
    }
  },
  methods: {
    increment() {
      // update component state
      this.count++
    }
  }
}
</script>


<template>
  <h1 :class="titleClass">Hello World!</h1>
  <button @click="increment">count is: {{ count }}</button>
</template>


<style>
.title {
  color: red;
}
</style>
