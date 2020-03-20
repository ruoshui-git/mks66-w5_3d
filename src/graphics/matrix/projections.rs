// perspectiveMatrix = function (fieldOfViewInRadians, aspectRatio, near, far) {
  
//     // Construct a perspective matrix
  
//     /*
//        Field of view - the angle in radians of what's in view along the Y axis
//        Aspect Ratio - the ratio of the canvas, typically canvas.width / canvas.height
//        Near - Anything before this point in the Z direction gets clipped (outside of the clip space)
//        Far - Anything after this point in the Z direction gets clipped (outside of the clip space)
//     */
  
//     var f = 1.0 / Math.tan(fieldOfViewInRadians / 2);
//     var rangeInv = 1 / (near - far);
 
//     return [
//       f / aspectRatio, 0,                          0,   0,
//       0,               f,                          0,   0,
//       0,               0,    (near + far) * rangeInv,  -1,
//       0,               0,  near * far * rangeInv * 2,   0
//     ];
//   }

// orthographicMatrix = function(left, right, bottom, top, near, far) {
  
//     // Each of the parameters represents the plane of the bounding box
  
//     var lr = 1 / (left - right);
//     var bt = 1 / (bottom - top);
//     var nf = 1 / (near - far);
	
//     var row4col1 = (left + right) * lr;
//     var row4col2 = (top + bottom) * bt;
//     var row4col3 = (far + near) * nf;
  
//     return [
//        -2 * lr,        0,        0, 0,
//              0,  -2 * bt,        0, 0,
//              0,        0,   2 * nf, 0,
//       row4col1, row4col2, row4col3, 1
//     ];
//   }