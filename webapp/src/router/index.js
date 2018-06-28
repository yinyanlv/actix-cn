import Vue from 'vue'
import Router from 'vue-router'
import Home from '../views/home/Home'
import HomePage from '../views/home/HomePage'
import HomeCategory from '../views/home/HomeCategory'
import HomeCategoryPage from '../views/home/HomeCategoryPage'
import Wiki from '../views/wiki/Wiki'
import Rust from '../views/wiki/dir/Rust'
import Actix from '../views/wiki/dir/Actix'
import Yew from '../views/wiki/dir/Yew'
import Explore from '../views/explore/Explore'
import Theme from '../views/theme/Theme'
import Post from '../views/new/Post'
import Create from '../views/new/Create'
import Access from '../views/user/Access'
import SignUp from '../views/user/SignUp'
import Center from '../views/user/Center'
import More from '../views/more/More'
import NotFound from '../views/notfound/NotFound'
Vue.use(Router)
export default new Router({
  mode: 'history',
  linkActiveClass: 'is-active',
  routes: [
    { path: '/', name: 'home', component: Home },
        { path: '/a/home/page/:number', name: 'home_page', component: HomePage },
        { path: '/a/home/:homecategory', name: 'homecategory', component: HomeCategory },
        { path: '/a/home/:homecategory/:number', name: 'homecategory_page', component: HomeCategoryPage },
    { path: '/a/wiki', name: 'wiki', component: Wiki },
        { path: '/a/wiki/rust', name: 'rust', component: Rust },
        { path: '/a/wiki/actix', name: 'actix', component: Actix },
        { path: '/a/wiki/yew', name: 'yew', component: Yew },
    { path: '/a/explore', name: 'explore', component: Explore },
    { path: '/a/:category/theme/:id', name: 'theme', component: Theme },
    { path: '/a/post', name: 'post', component: Post },
    { path: '/a/create', name: 'create', component: Create },
    { path: '/a/access', name: 'access', component: Access },
    { path: '/a/signup', name: 'signup', component: SignUp },
    { path: '/a/user/:id', name: 'user', component: Center },
    { path: '/a/more', name: 'more', component: More },
    { path: '*', name: 'notfound', component: NotFound }
  ]
})