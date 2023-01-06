// 关键字 ：let fun class enum interface for if else break continue vararg native is
// 此数据类型可被迭代
interface Iterable<T>{
    fun iter():Iter<V>
}
// 程序终止符号
native fun exit(Number,Text = '')
// 不可变数组
native class Array<V>(vararg data:V){
    // 查询数组长度
    native fun len():Number
    // 获取数组的某一个
    native fun get(Number):V
    // 设置数组
    native fun set(Number,V):V
}

// 定义的迭代器
interface Iter<V>{
    fun has_next():Bool
    fun next():V
}

class MapEntity<K,V>(key:K,value:V)

let entity = MapEntity('12','34')
