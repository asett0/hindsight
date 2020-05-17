FROM python:3.8.3-alpine3.11
COPY api/requirements.txt /app/api/requirements.txt
WORKDIR /app/api/
RUN pip install -r requirements.txt
COPY api/src/ /app/api/src/
CMD ["python3","src/main.py","--host=0.0.0.0"]