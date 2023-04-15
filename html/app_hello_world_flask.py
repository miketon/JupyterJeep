from flask import Flask
from flask_sqlalchemy import SQLAlchemy

refresh_count = 0
app = Flask(__name__)
app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///visit_count.db"
app.config["SQLALCHEMY_TRACK_MODIFICATIONS"] = False
db = SQLAlchemy(app)


class Counter(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    count = db.Column(db.Integer, nullable=False)


# Setup the application context BEFORE creating the database tables
with app.app_context():
    db.create_all()


@app.route("/")
def home():
    global refresh_count
    counter = Counter.query.first()
    if counter is None:
        counter = Counter(count=0)
        db.session.add(counter)
    counter.count = counter.count + 1
    db.session.commit()

    refresh_count = refresh_count + 1
    return f"Hello, world! This page has been db visited {counter.count} and refreshed {refresh_count} times."


if __name__ == "__main__":
    app.run(debug=True, port=5019)
