# 短版视频的 Veo 提示词

对应 `promo-video-script.zh-CN.md` 中短版 0:00–0:09 的人物镜头，以及 0:21–0:26 的手表镜头（可选）。

## 使用方法

1. 先用第 0 步的提示词生成一张**定妆图**（9:16）。在 Gemini 里生成，或在 Flow 里用 Imagen 生成都可以。
2. 在 **Flow** 里选「Ingredients to Video」，把定妆图作为人物参考，再粘贴下面各镜头的提示词。这样所有镜头里是同一个人。
3. 画幅选 **9:16**。每次生成 2–4 条，挑最好的一条。
4. Veo 一次生成 8 秒，在 CapCut 里剪出需要的那一段就行。
5. 所有提示词都要求**不出现对白、不出现音乐、屏幕上没有可读的文字**。原因：旁白和配乐后期另加；AI 生成的屏幕文字会是乱码，真实界面用录屏叠上去。

每个提示词里的人物描述都一字不差地重复，这样有助于保持形象一致。想换人物，只要统一替换这一段：

> a programmer in their late twenties, East Asian, short messy black hair, no glasses, wearing a dark grey hoodie

---

## 0. 定妆图（Imagen / Gemini）

```
Vertical 9:16 portrait photo. A programmer in their late twenties, East Asian, short messy black hair, no glasses, wearing a dark grey hoodie, sitting at a desk late at night. The room is dark; the only light is the cool blue-white glow of a computer monitor on their face. Tired expression, slightly slouched. Cinematic, realistic, shallow depth of field, 35mm film look, muted colors. No text anywhere.
```

## 1. 深夜点掉提醒（0:00–0:04，取约 2.5 秒）

剪辑时接一段约 1.5 秒的真实录屏：系统右下角弹出普通的休息提醒，鼠标点「关闭」。

```
Vertical 9:16, cinematic realistic shot. Late at night in a dark apartment room. A programmer in their late twenties, East Asian, short messy black hair, no glasses, wearing a dark grey hoodie, sits at a desk, face lit only by the cold blue glow of a monitor (the monitor itself is off-screen or out of focus). Medium close-up from slightly in front and to the side. Their eyes flick briefly to the corner of the screen, then, without any change of expression, their hand on the mouse makes a quick dismissive click, and their gaze returns to the center of the screen. Indifferent, routine gesture. Very slow push-in. Muted colors, shallow depth of field, 35mm film grain.
Audio: quiet room tone, soft computer fan hum, a single light mouse click. No dialogue, no music.
Avoid: readable text, UI elements, logos, subtitles.
```

## 2a. 眼睛（0:04–0:09 的第 1 段，取约 1.5 秒）

```
Vertical 9:16, extreme close-up of the eyes of a programmer in their late twenties, East Asian, short messy black hair, no glasses, wearing a dark grey hoodie. Late at night, lit only by cold blue monitor light. The eyes are visibly bloodshot and watery, the person blinks hard and slowly several times, squinting as if struggling to focus, then raises a small bottle of eye drops and puts a drop into one eye, flinching slightly. Realistic skin texture, shallow depth of field, cinematic, muted colors, 35mm film grain.
Audio: quiet room tone, a soft blink-like sound, faint computer fan. No dialogue, no music.
Avoid: readable text, logos, brand names on the bottle, subtitles.
```

## 2b. 头晕脑胀（0:04–0:09 的第 2 段，取约 1.5 秒）

```
Vertical 9:16, medium close-up of a programmer in their late twenties, East Asian, short messy black hair, no glasses, wearing a dark grey hoodie, sitting at a desk late at night, lit by cold blue monitor light. Eyes closed, they press both hands to their temples and slowly rub them, wincing, then roll their stiff neck to one side. The camera sways very slightly as if handheld, and the focus slowly drifts soft and back again, conveying dizziness and pressure. Cinematic, realistic, muted colors, 35mm film grain.
Audio: low muffled room tone with a faint high-pitched ringing in the ears, slow breathing. No dialogue, no music.
Avoid: readable text, logos, subtitles.
```

## 2c. 效率下降（0:04–0:09 的第 3 段，取约 1.5 秒）

剪辑时，可以把真实的终端报错录屏放大、虚化，叠在这一镜的屏幕区域，或者跟这一镜快速交叉剪辑。

```
Vertical 9:16, over-the-shoulder shot from behind a programmer in their late twenties, East Asian, short messy black hair, no glasses, wearing a dark grey hoodie, sitting at a desk late at night. The monitor in front of them is heavily out of focus, showing only blurry blocks of light with a faint red tint, like error messages. They type a few keys, stop, then lean back, run both hands through their hair and exhale in frustration, staring blankly at the screen. Dark room, cold monitor light, cinematic, realistic, muted colors, 35mm film grain.
Audio: a short burst of keyboard typing, then silence, a heavy sigh, faint computer fan. No dialogue, no music.
Avoid: readable text or code on the screen, logos, subtitles.
```

黑屏和心跳（这一段最后约 1 秒）不用 Veo，直接在 CapCut 里做：纯黑画面，配心跳音效，再加字幕「程序员猝死，并不少见。」

## 3. 手表轻震（可选，0:21–0:26，取约 2 秒）

剪辑时，在表盘上叠一张真实的 ntfy 通知截图。

```
Vertical 9:16, close-up of a wrist wearing a plain black smartwatch with a blank, softly glowing screen, on a person in a dark grey hoodie standing on a sunny balcony, holding a glass of water. The watch gently vibrates; the person lifts their wrist, glances at it and smiles slightly, relaxed. Warm daylight, soft background bokeh of plants and sky, cinematic, realistic, warm colors.
Audio: a short soft vibration buzz, light outdoor ambience with distant birds. No dialogue, no music.
Avoid: readable text on the watch screen, brand logos, subtitles.
```

---

## 常见问题

- **人物脸不一致**：确认在 Ingredients to Video 里放了定妆图。还不行的话，就把定妆图作为首帧，改用「Frames to Video」。
- **屏幕上出现乱码文字**：在提示词里强调 `heavily out of focus`，或者把机位改成看不到屏幕的角度。
- **Veo 自己加了背景音乐或说话声**：重新生成，或者在 CapCut 里把这一段的原声删掉，只保留需要的环境音。
- **9:16 效果不好**：改用 16:9 生成，再在 CapCut 里裁成竖屏。人物要放在画面中间，提示词里加 `subject centered`。
