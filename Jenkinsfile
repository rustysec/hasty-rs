pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh 'cargo build'
      }
    }
    stage('Run Tests') {
      steps {
        sh 'chmod +x ./tests.sh && sh ./tests.sh'
      }
    }
  }
}