该库为操作系统课程设计时所编写，为了避免被认定为抄袭所以特此声明。

测试结果：

```
小规模数据测试（扇区号1~512，32个请求）：
	随机初始磁头位置：448
	随机磁盘访问队列：196, 275, 262, 185, 228, 186, 302, 335, 307, 48, 324, 283, 21, 13, 45, 258, 190, 332, 340, 237, 497, 130, 49, 478, 74, 294, 506, 129, 105, 149, 240, 251, 

    先来先服务算法：
	算法给出的扫描队列：196, 275, 262, 185, 228, 186, 302, 335, 307, 48, 324, 283, 21, 13, 45, 258, 190, 332, 340, 237, 497, 130, 49, 478, 74, 294, 506, 129, 105, 149, 240, 251, 
	平均寻道长度：144.22

	最短寻道时间优先算法：
	算法给出的扫描队列：478, 497, 506, 340, 335, 332, 324, 307, 302, 294, 283, 275, 262, 258, 251, 240, 237, 228, 196, 190, 186, 185, 149, 130, 129, 105, 74, 49, 48, 45, 21, 13, 
	平均寻道长度：17.22

	扫描算法：
	算法给出的扫描队列：478, 497, 506, 340, 335, 332, 324, 307, 302, 294, 283, 275, 262, 258, 251, 240, 237, 228, 196, 190, 186, 185, 149, 130, 129, 105, 74, 49, 48, 45, 21, 13, 
	平均寻道长度：17.22

	循环扫描算法：
	算法给出的扫描队列：478, 497, 506, 13, 21, 45, 48, 49, 74, 105, 129, 130, 149, 185, 186, 190, 196, 228, 237, 240, 251, 258, 262, 275, 283, 294, 302, 307, 324, 332, 335, 340, 
	平均寻道长度：27.44

大规模数据测试：（扇区号1~65535，8192个请求，运行32轮，求平均值）
	先来先服务算法的平均寻道长度：21841.65
	先来先服务算法的平均寻道长度：12.75
	先来先服务算法的平均寻道长度：12.02
	先来先服务算法的平均寻道长度：16.00
```