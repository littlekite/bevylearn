**bevylearn 一起学习bevy**
## 项目名称  绝地求生

#### 基本描述

1. 2D像素
2. 固定场景、摄像机位
<s>3. 前期对战AI 后期添加多人对战</s>
4. 仅支持 左右操作和发射子弹  没有上下移动
5. 有血条  中弹减少血量  血条为0 时默认失败

#### 中立区域

```seq
中间固定区域会随机生成（空投）血包，子弹，武器
也会随机出现中立敌人，向上或向下都有可能攻击
1:按E，派遣搬运兵 前往中立区域将（血包，子弹，武器）运回来
2:搬运兵，血包，子弹，武器有固定防护血量，会被对面子弹打爆
```

#### 效果图
![markdown](https://github.com/littlekite/bevylearn/blob/main/demo.png)