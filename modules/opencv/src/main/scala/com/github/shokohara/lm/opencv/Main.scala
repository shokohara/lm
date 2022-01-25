package com.github.shokohara.lm.opencv

import org.opencv.core.CvType
import sourcecode.Name
import org.opencv.core.{Core, MatOfRect, Point, Scalar, Size}
import org.opencv.core.Mat
import org.opencv.imgcodecs.Imgcodecs
import org.opencv.imgproc.Imgproc
import org.opencv.objdetect.CascadeClassifier
import org.opencv.core.MatOfPoint
import org.opencv.core.Mat
import better.files.*
import better.files.Dsl.*

object Main {
//  (pwd / `..` / `..` / "tmp" / "dev").clear()
//  System.loadLibrary(Core.NATIVE_LIBRARY_NAME)
  def main(args: Array[String]): Unit =
    println(pwd.pathAsString)
    System.load("/usr/local/share/java/opencv4/libopencv_java455.dylib")
//    new Generator("../..")
  def archive = {
    def canny(image: Mat, t1: Int, t2: Int) = {
      Imgproc.cvtColor(image, image, Imgproc.COLOR_RGB2GRAY)
      Imgproc.GaussianBlur(image, image, new Size(3, 3), 0)
      Imgproc.Canny(image, image, t1, t2)
    }
    def f(image: Mat, t1: Int, t2: Int) = {
//      canny(image, t1, t2)
      Imgproc.cvtColor(image, image, Imgproc.COLOR_RGB2GRAY)
//      val result: Mat = new Mat(image.rows(), image.cols(), CvType.CV_32FC1)
      Imgproc.threshold(image, image, 0, 255, Imgproc.THRESH_BINARY | Imgproc.THRESH_OTSU)
      //      val hierarchy = new Mat()
//      val contours: java.util.List[MatOfPoint] = new java.util.ArrayList[MatOfPoint]()
//      Imgproc.findContours(image, contours, hierarchy, Imgproc.RETR_TREE, Imgproc.CHAIN_APPROX_NONE)
//      Imgproc.drawContours(image, contours, -1, new Scalar(0, 255, 0), 3)
      Imgcodecs.imwrite(
        s"${new java.io.File(".").getAbsoluteFile().getParent()}/../../tmp/dev/${Name()}${image.width()}$t1$t2.png",
        image)
    }

    val grayCh =
      Imgcodecs.imread(s"${new java.io.File(".").getAbsoluteFile().getParent()}/resource/validation/7z/a.png")
    Imgproc.cvtColor(grayCh, grayCh, Imgproc.COLOR_RGB2GRAY)
    Imgproc.threshold(grayCh, grayCh, 100, 255, Imgproc.THRESH_BINARY_INV | Imgproc.THRESH_OTSU)
    Imgcodecs.imwrite(
      s"${new java.io.File(".").getAbsoluteFile().getParent()}/../../tmp/dev/${Name()}${grayCh.width()}.png", grayCh)
    (4 to 4).map { i =>
      {
        val im = Imgcodecs.imread(s"${new java.io.File(".").getAbsoluteFile().getParent()}/../../tmp/res/ss.png")
        f(im, i * 10, i * 2 * 10)
      }
      val image = Imgcodecs.imread(s"${new java.io.File(".").getAbsoluteFile().getParent()}/../../tmp/res/ss.png")
      //  Imgproc.Sobel(image, image, CvType.CV_32F, 1, 1, 1, 0.1)
      //  Imgproc.Laplacian(image, image, CvType.CV_32F)
//      canny(image, i * 10, i * 2 * 10)
      f(image, i, i)
      Imgcodecs.imwrite(s"${new java.io.File(".").getAbsoluteFile().getParent()}/../../tmp/dev/debugimage$i.png", image)
//      val template = Imgcodecs.imread(s"${new java.io.File(".").getAbsoluteFile().getParent()}/../../tmp/res/ch.png")
//      //  Imgproc.GaussianBlur(template, template, new Size(2, 2), 2)
//      //  Imgproc.Laplacian(template, template, CvType.CV_32F)
//      //  Imgproc.Canny(image, image, 100, 200)
//      canny(template, i * 10, i * 2 * 10)
//      Imgcodecs.imwrite(s"${new java.io.File(".").getAbsoluteFile().getParent()}/../../tmp/dev/debugtemplate$i.png", template)
      val template = grayCh

      val result: Mat = new Mat(image.rows() - template.rows() + 1, image.cols() - template.cols() + 1, CvType.CV_32FC1)
      val res = Imgproc.matchTemplate(image, template, result, Imgproc.TM_CCOEFF_NORMED)
      Imgproc.threshold(result, result, 0.5, 1.0, Imgproc.THRESH_TOZERO)

      (0 until result.rows()).map { i =>
        (0 until result.cols()).map { j =>
          if (result.get(i, j).head > 0) {
            println((i, j))
            Imgproc.rectangle(image, new Point(j, i), new Point(j + template.cols(), i + template.rows()),
              new Scalar(0, 0, 255))
          }
        }
      }
      Imgcodecs
        .imwrite(s"${new java.io.File(".").getAbsoluteFile().getParent()}/../../tmp/dev/faceDetection$i.png", image)
    }
  }
}
