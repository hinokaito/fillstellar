// 頂点シェーダーの出力（フラグメントシェーダーへの入力）
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {
    var out: VertexOutput;

    // 頂点バッファが無いので、コード内で座標を定義します
    var positions = array<vec2<f32>, 3>(
        vec2<f32>( 0.0,  0.5), // 上
        vec2<f32>(-0.5, -0.5), // 左下
        vec2<f32>( 0.5, -0.5)  // 右下
    );

    // インデックス(0, 1, 2)に応じて座標を取り出す
    let p = positions[in_vertex_index];
    
    // wgpuのクリップ座標系に変換 (Z=0.0, W=1.0)
    out.clip_position = vec4<f32>(p, 0.0, 1.0);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 赤色を返す (R, G, B, A)
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}






// // 構造体定義
// struct VertexOutput {
//     @builtin(position) position: vec4f,
//     @location(0) uv: vec2f,
// };
//  pos = array():

// @binding(0) @group(0) var<uniform> frame: u32;
// @vertex
// fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4f {
//   if true {
//     const pos = array(
//       vec2(0.0, 0.5),
//       vec2(0.5, -0.5),
//       vec2(0.5, -0.5)
//     );
//   } else {
//     const pos = array(
//       vec2(0.0, 0.5),
//       vec2(0.5, -0.5),
//       vec2(0.5, -0.5)
//     );
//   }
  

//   return vec4f(pos[vertex_index], 0, 1);
// }

// @fragment
// fn fs_main() -> @location(0) vec4f {
//   return vec4(1, sin(f32(frame) / 128), 0, 1);
// }

// // @vertex
// // fn vs_main(@builtin(vertex_index) vertexIndex: u32) -> VertexOutput {
// //     var pos = array<vec2f, 3>(
// //         vec2f(-1.0, -1.0),
// //         vec2f( 3.0, -1.0),
// //         vec2f( 1.0,  3.0)
// //     );

// //     var output: VertexOutput;

// //     let xy = pos[vertexIndex];
// //     output.position = vec4f(xy, 0.0, 1.0);
// //     output.uv = xy;

// //     return output;
// // }

// // @fragment
// // fn fs_main(input: VertexOutput) -> @location(0) vec4f {
// //     let dist = length(input.uv);

// //     let circle = smoothstep(0.5, 0.49, dist);

// //     return vec4f(circle, circle, circle, 1.0);
// // }






// // // 2. Vertex Shader（キャンバスの準備）
// // @vertex
// // fn vs_main(@builtin(vertex_index) vertexIndex: u32) -> VertexOutput {
// //   var pos = array<vec2f, 3>(
// //     vec2f(-1.0, -1.0),
// //     vec2f( 3.0, -1.0),
// //     vec2f(-1.0,  3.0)
// //   );

// //   var output: VertexOutput;
// //   let xy = pos[vertexIndex];
  
// //   output.position = vec4f(xy, 0.0, 1.0);
// //   output.uv = xy; // 座標情報をフラグメントへ転送
  
// //   return output;
// // }

// // // 3. Fragment Shader（星の描画）
// // @fragment
// // fn fs_main(input: VertexOutput) -> @location(0) vec4f {
// //   // 受け取ったUV座標を使用（中心はすでに0,0になっている）
// //   let uv = input.uv;
  
// //   // --- 星の計算ロジック ---
// //   let dist = length(uv);
// //   let angle = atan2(uv.y, uv.x);

// //   // 半径を波打たせる（星の形）
// //   // cos(angle * 5.0) で5角星
// //   let star_radius = 0.3 + 0.15 * cos(angle * 5.0);

// //   // 描画範囲の判定
// //   // smoothstepで輪郭をなめらかに切り取る
// //   let shape = smoothstep(star_radius + 0.01, star_radius - 0.01, dist);
  
// //   // 色付け
// //   let star_color = vec3f(1.0, 0.9, 0.2); // 黄色
// //   let bg_color = vec3f(0.1, 0.1, 0.2);   // 紺色

// //   // 背景と星を合成
// //   let final_color = mix(bg_color, star_color, shape);

// //   return vec4f(final_color, 1.0);
// // }














// // @fragment
// // fn fs_main(input: VertexOutput) -> @location(0) vec4f {
// //     // 画面の解像度
// //     let resolution = vec2f(800.0, 800.0);
// //     // ピクセルの位置を-1.0 ~ 1.0に正規化
// //     // (input.position.xyは現在のピクセルの座標)
// //     let uv = (input.position.xy - resolution * 0.5) / (resolution.y * 0.5);

// //     return vec4f(uv.x, uv.y, 0.0, 1.0);
// // } 


// // @fragment
// // fn fs_main(input: VertexOutput) -> @location(0) vec4f {
// //     let resolution = vec2f(800.0, 800.0);
// //     let uv = (input.position.xy - resolution * 0.5) / (resolution.y * 0.5);
    
// //     // lengthで原点(0, 0)からの距離を測る
// //     let dist = length(uv);

// //     // 距離が0.5より小さければ白、そうでなければ黒
// //     // step(edge, x)はx<edgeなら0.0, x>=edgeなら1.0を返す
// //     let color_value = 1.0 - step(0.5, dist);

// //     return vec4f(color_value, color_value, color_value, 1.0);
// // }
 
// // // 頂点シェーダー: 三角形の3つの頂点の位置を決める
// // @vertex
// // fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
// //     // 3つの頂点座標をハードコード(直接書き込み)する
// //     // 本来はRust側からデータを送るが、今回は一番簡単な方法でやる
// //     var pos = array<vec2<f32>, 3>(
// //         vec2<f32>( 0.0,  0.5), // 上
// //         vec2<f32>(-0.5, -0.5), // 左下
// //         vec2<f32>( 0.5, -0.5)  // 右下
// //     );

// //     // 選ばれた頂点の座標を返す
// //     return vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
// // }

// // // フラグメントシェーダー: ピクセルの色を決める
// // @fragment
// // fn fs_main() -> @location(0) vec4<f32> {
// //     // 赤色 (R=1.0, G=0.0, B=0.0, A=1.0)
// //     return vec4<f32>(1.0, 0.0, 0.0, 1.0);
// // }