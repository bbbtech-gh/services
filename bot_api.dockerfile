FROM python:3
WORKDIR /usr/src/app
COPY bot_api .
RUN pip install -r requirements.txt
# COPY *.py .
CMD [ "uvicorn", "index:app", "--port", "8000", "--host", "0.0.0.0" ]