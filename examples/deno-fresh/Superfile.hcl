project = "deno-fresh"

service "deno" {
  type = "docker"
  command = "./dev.ts"
  working_dir = "."
  description = "Deno example app"
  depends_on = []
  env = {}
  autostart = true
  autorestart = false
  namespace = "deno_namespace"
  stdout = "/tmp/deno-stdout.log"
  stderr = "/tmp/deno-stderr.log"
  port = 8000

  #use "flox" {
  #  environment = ".#deno-fresh"
  #}

  use "docker" {
  # volumes = ["./data:/data"]
  }
}
