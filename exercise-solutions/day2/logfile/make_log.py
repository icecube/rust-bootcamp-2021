from datetime import datetime, timedelta
import random

users = ['dglo', 'nega', 'blinkdog', 'kjmeagher', 'olivas', 'ric-evans', 'clark2668', 'mskarl', 'austinschneider', 'colemanalan', 'jnbellinger', 'aludez']

modules = ['root', 'tracker', 'handlers', 'memory_tracker', 'cpuhog']

def MemError():
    b = random.randint(100000,10000000)
    return f'memory error: {b} KB used'
def CPUError():
    cores = (random.random()+3)*10
    return f'cpu error: {cores:.2f} used'
def Timeout():
    t = random.random()*3600
    return f'timeout: {t} seconds overdue'
def Processed():
    t = random.randint(1,100)
    return f'{t} jobs processed in last minute'

messages = {
    'ERROR': [MemError, CPUError],
    'WARNING': [Timeout],
    'INFO': [Processed],
}


with open("log", 'w') as f:
    def log(time, level, module, user, message):
        print(f'{time} {level} {module} - {user}: {message}', file=f)

    date = datetime.utcnow()-timedelta(hours=24)
    enddate = datetime.utcnow()

    while date < enddate:
        level = random.choices(list(messages), weights=[1,3,10], k=1)[0]
        log(date.isoformat(), level, random.choice(modules), random.choice(users), random.choice(messages[level])())
        date += timedelta(seconds=random.random()*100)
