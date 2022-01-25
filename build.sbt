import com.typesafe.sbt.packager.docker._
Global / onChangedBuildSource := ReloadOnSourceChanges
ThisBuild / scalaVersion := "3.1.1"
ThisBuild / scalafixDependencies += "com.github.liancheng" %% "organize-imports" % "0.5.0"
ThisBuild / organization := "com.github.shokohara.mj"
ThisBuild / organizationName := "Underscore Co., Ltd."
val munit = "org.scalameta" %% "munit" % "0.7.29" % Test
val buildInfo = (project in file("modules/build-info")).enablePlugins(BuildInfoPlugin)
val mUnitFramework = new TestFramework("munit.Framework")
val MUnitFramework = new TestFramework("munit.Framework")
val Slow = config("slow").extend(Test)
val All = config("slow").extend(Test)
val common = project in file("modules/common")
val opencv = (project in file("modules/opencv"))
  .configs(Slow, All)
  .settings(
    semanticdbEnabled := true,
    semanticdbVersion := scalafixSemanticdb.revision,
    inConfig(Slow)(Defaults.testTasks),
    inConfig(All)(Defaults.testTasks),
    All / testOptions := List(),
    Test / testOptions += Tests.Argument(MUnitFramework, "--exclude-tags=Slow"),
    Slow / testOptions -= Tests.Argument(MUnitFramework, "--exclude-tags=Slow"),
    Slow / testOptions += Tests.Argument(MUnitFramework, "--include-tags=Slow"),
//    run / fork := true,
//    test / run / fork := true,
    fork := true,
    run / javaOptions += "-Djava.library.path=/usr/local/share/java/opencv4",
//    unmanagedBase := new java.io.File("."),
//    Compile / unmanagedResourceDirectories += new java.io.File("/usr/local/Cellar/opencv/4.5.4_3/share/java/opencv4"),
    libraryDependencies ++= Seq("com.lihaoyi" %% "sourcecode" % "0.2.7") ++ Seq(
      "org.typelevel" %% "munit-cats-effect-3" % "1.0.6" % Test,
      "org.scala-lang.modules" %% "scala-parallel-collections" % "1.0.4",
      ("com.chatwork" %% "scala-ulid" % "1.0.24").cross(CrossVersion.for3Use2_13),
//      "com.badlogicgames.gdx" % "gdx" % "1.10.0",
//      "com.softwaremill.sttp.tapir" %% "tapir-core" % "0.20.0-M5",
//      "com.softwaremill.sttp.tapir" %% "tapir-json-circe" % "0.20.0-M5",
//      "com.softwaremill.sttp.tapir" %% "tapir-openapi-docs" % "0.20.0-M5",
//      "com.softwaremill.sttp.tapir" %% "tapir-sttp-client" % "0.20.0-M5",
//      "com.softwaremill.sttp.tapir" %% "tapir-openapi-circe-yaml" % "0.20.0-M5",
//      "com.softwaremill.sttp.tapir" %% "tapir-http4s-server" % "0.20.0-M5",
//      "org.http4s" %% "http4s-blaze-server" % "0.23.7",
//      "io.github.vigoo" %% "prox-fs2-3" % "0.7.3" % Test,
//      "io.circe" %% "circe-fs2" % "0.14.0" % Test,
//      "org.gnieh" %% "diffson-circe" % "4.1.1" % Test,
      "com.somainer" %% "scala3-nameof" % "0.0.1" % Provided,
//      "software.amazon.awscdk" % "aws-cdk-lib" % cdkVersion,
//      "commons-codec" % "commons-codec" % "1.15",
//      "net.lingala.zip4j" % "zip4j" % "2.9.1",
      ("com.github.pathikrit" %% "better-files" % "3.9.1").cross(CrossVersion.for3Use2_13)
    ) ++ Seq("", "-std", "-kernel").map(s => "org.typelevel" %% s"cats-effect$s" % "3.2.9"),
    assembly / assemblyMergeStrategy := {
      case "module-info.class" => MergeStrategy.discard
      case x                   => (assembly / assemblyMergeStrategy).value(x)
    },
    assembly / assemblyJarName := name.value + ".jar",
    dockerBaseImage := "mj:latest",
    dockerAdditionalPermissions += (DockerChmodType.UserGroupWriteExecute, "/tmp")
  )
  .dependsOn(buildInfo, common)
  .enablePlugins(DockerPlugin, JavaAppPackaging)
val benchmark = (project in file("modules/benchmark"))
  .settings(libraryDependencies += ("com.github.pathikrit" %% "better-files" % "3.9.1").cross(CrossVersion.for3Use2_13))
  .enablePlugins(JmhPlugin)
  .dependsOn(opencv)
val awsSdkVersion = "2.17.103"
val generator = (project in file("modules/react"))
  .settings(
    libraryDependencies += ("com.sksamuel.scrimage" %% "scrimage-scala" % "4.0.25").cross(CrossVersion.for3Use2_13),
    libraryDependencies += ("com.github.pathikrit" %% "better-files" % "3.9.1").cross(CrossVersion.for3Use2_13),
    semanticdbEnabled := true,
    semanticdbVersion := scalafixSemanticdb.revision,
    nativeLinkStubs := true
  )
//  .enablePlugins(ScalaNativePlugin)
