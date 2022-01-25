package jmh

import java.util.UUID
import java.util.concurrent.TimeUnit

import com.chatwork.scala.ulid.{ULID => cwULID}
import org.openjdk.jmh.annotations.{Benchmark, BenchmarkMode, Mode, OutputTimeUnit, Scope, State}
import better.files.*
import better.files.Dsl.*

@State(Scope.Benchmark)
@BenchmarkMode(Array(Mode.SampleTime))
@OutputTimeUnit(TimeUnit.SECONDS)
class MyBenchmark {

  System.load("/usr/local/share/java/opencv4/libopencv_java455.dylib")

  @Benchmark
  def detectTehai: Unit = DetectMyDoubleNakis
    .detectTehai(pwd / `..` / "opencv",
      (_: File) / "src" / "test" / "resources" / "Screen Shot 2022-01-19 at 17.18.13.png", false)

  @Benchmark
  def detectNakis: Unit = DetectMyDoubleNakis
    .detectNakis(pwd / `..` / "opencv",
      (_: File) / "src" / "test" / "resources" / "Screen Shot 2022-01-19 at 17.18.13.png", false)

  @Benchmark
  def detectZicha: Unit = DetectMyDoubleNakis
    .detectZicha(pwd / `..` / "opencv",
      (_: File) / "src" / "test" / "resources" / "Screen Shot 2022-01-19 at 17.18.13.png", false)
}
