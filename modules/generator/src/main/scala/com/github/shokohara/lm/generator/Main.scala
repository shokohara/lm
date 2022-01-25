package com.github.shokohara.mj.generator

import better.files.Dsl.*
import better.files.*
import javax.imageio.ImageIO
import java.awt.Rectangle
import java.awt.Robot
import java.awt.Toolkit
import java.awt.image.BufferedImage
import java.io.File
import com.sksamuel.scrimage.*
import com.sksamuel.scrimage.nio.PngWriter

object Main {
  def main(args: Array[String]): Unit = {
    val a = ImmutableImage.loader().fromResource("/hand_ui.png")
    (((0, 0, "0s") +: (1 to 9).map(n => (n, 0, s"${n}s"))) ++
      ((0, 1, "0m") +: (1 to 9).map(n => (n, 1, s"${n}m"))) ++
      ((0, 2, "0s") +: (1 to 9).map(n => (n, 2, s"${n}p"))) ++
      ((0, 3, "1z") :: (1, 3, "2z") :: (2, 3, "3z") :: (3, 3, "4z") :: (4, 3, "5z") :: (5, 3, "6z") :: (6, 3,
        "7z") :: Nil))
      .foreach { (x, y, n) =>
        val w = 80
        val h = 129
        val marginTop = 20
        val marginX = 3
        val writer = PngWriter.NoCompression
        a.subimage(x * w + marginX, y * h + marginTop, w - marginX * 2, h - marginTop - marginX)
          .output(writer, (pwd / "modules" / "generator" / "output" / s"$n.png").toJava)
      }
  }
}
