from flask import Flask

app = Flask(__name__)

@app.route('/')
def hello():
    docker_ascii = '''\
          ##         .
    ## ## ##        ==
 ## ## ## ## ##    ===
/"""""""""""""""""\___/ ===
{                       /  ===-
\______ O           __/
 \    \         __/
  \____\_______/

Hello from Docker!
'''
    return '<pre>' + docker_ascii + '</pre>'

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080)
