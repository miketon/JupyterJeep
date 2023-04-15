from flask import Flask

app = Flask(__name__)
visit_count = 0

@app.route('/')
def home():
    global visit_count
    visit_count += 1
    return f"Hello, world! This page has been visited {visit_count} times."

if __name__ == "__main__":
    app.run(debug=True)