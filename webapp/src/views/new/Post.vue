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
                                            <option v-bind:value="category_name" v-for="(category_name, index) in category_names_admin" :key="index">
                                                    {{category_name}}
                                            </option>
                                        </select>
                                        <select v-else name="category_name" v-model="CategoryName" id="category-control">
                                            <option v-bind:value="category_name" v-for="(category_name, index) in category_names" :key="index">
                                                    {{category_name}}
                                            </option>
                                        </select>
                                </span>
                                <span id="title">
                                        <input type="text" name="title" v-model="Title" placeholder="Please input title">
                                </span>
                            </div>    
                            <div id="editor">
                                <mavon-editor name="content" v-model="Content" :ishljs = "true" style="height: 100%;" :toolbars="set"></mavon-editor>
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
import { mavonEditor } from 'mavon-editor'
import 'mavon-editor/dist/css/index.css'
import URLprefix from '../../config'
import Side from '../../components/side/Side'
export default {
    name: 'post',
    components: {
        "side": Side,
        mavonEditor
    },
    data () {
        return {
            username: '',
            category_names: '',
            category_names_admin: '',
            CategoryName: '',
            Title: '',
            Content: '',
            set:{
                bold: true, // 粗体
                italic: true, // 斜体
                header: true, // 标题
                underline: true, // 下划线
                strikethrough: true, // 中划线
                mark: true, // 标记
                quote: true, // 引用
                ol: true, // 有序列表
                ul: true, // 无序列表
                link: true, // 链接
                code: true, // code
                trash: true, // 清空
                table: true, // 表格
                fullscreen: true, // 全屏编辑
                alignleft: true, // 左对齐
                aligncenter: true, // 居中
                alignright: true, // 右对齐
                preview: true, // 预览
                help: true, // 帮助

                superscript: false, // 上角标
                subscript: false, // 下角标
                undo: false, // 上一步
                redo: false, // 下一步
                imagelink: false, // 图片链接
                readmodel: false, // 沉浸式阅读
                htmlcode: false, // 展示html源码
                save: false, // 保存（触发events中的save事件）
                navigation: false, // 导航目录
                subfield: false, // 单双栏模式
            }
        }
    },
    mounted: function() {
        let md = mavonEditor.getMarkdownIt() 
        var username = JSON.parse(sessionStorage.getItem('signin_user')).username
        this.username = username
              fetch(URLprefix + 'api/categorys', {
                  method: 'GET',
              }).then(response => response.json())
              .then(json => {
                    this.categorys = json.categorys
                    let category_names_admin = []
                    let category_names = []
                    this.categorys.map((item) => category_names_admin.push(item.category_name))
                    for (let index = 0; index < category_names_admin.length; index++) {
                        if (category_names_admin[index] == 'office') category_names_admin[index] = '官方'
                        if (category_names_admin[index] == 'blog') category_names_admin[index] = '博客'
                        if (category_names_admin[index] == 'faq') category_names_admin[index] = '问答'
                        if (category_names_admin[index] == 'share')  category_names_admin[index] = '分享'
                        if (category_names_admin[index] == 'job') category_names_admin[index] = '招聘'
                    }
                    this.category_names_admin = category_names_admin
                    category_names_admin.filter((item) => { if (item != '官方') category_names.push(item)})
                    this.category_names = category_names
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
                    this.$router.push('/')
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
    background-color:#f5fdfa;
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
#editor {
    margin: auto;
    height: 444px;
}
form #new button {
    margin-top: 0.3rem;
    width:63px; 
    line-height:25px;
    background-color:#FFFFFF;
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
        background-color: #FFFFFF;
    }
    #container #side {
        flex: 1;
    }
}
</style>