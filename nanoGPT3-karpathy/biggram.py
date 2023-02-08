import torch
import torch.nn as nn
from torch.nn import functional as F
torch.manual_seed(1337)

# hyperparameters
BATCH_SIZE = 32 # how many INDEPENDENT samples to process in parallel
BLOCK_SIZE = 8 # what is the maximum context length for predictions
MAX_ITERS = 3000
EVAL_INTERVAL = 300
LEARNING_RATE = 1e-3
# Allows you to run on GPU if you have one
DEVICE = 'cuda' if torch.cuda.is_available() else 'cpu'
EVAL_ITERS = 200
VALIDATION_SPLIT = "validation_split"

# -------------

# wget https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt
with open('input.txt', 'r', encoding='utf-8') as f:
    text = f.read()

# here are all the unique characters that occur in the text
chars = sorted(list(set(text)))
vocab_size = len(chars)
# create a mapping from character to integers
stoi = {ch:i for i,ch in enumerate(chars)} # string to int
itos = {i:ch for i,ch in enumerate(chars)} # int to string

# encode : take a string and output a list of integers
encode = lambda input_string: [stoi[char] for char in input_string]
# decode : take a list of integers and output a string
decode = lambda input_int_list: ''.join([itos[int] for int in input_int_list])

# train and test splits
data = torch.tensor(encode(text), dtype=torch.long)
# first 90% of data is for training, remaining 10% is for testing
n = int(len(data) * 0.9)
train_data = data[:n]
val_data = data[n:]

# data loading
def get_batch(split=VALIDATION_SPLIT):
    # generate a small batch of data of inputs (x) and targets (y)
    data = val_data if split == VALIDATION_SPLIT else train_data
    ix = torch.randint(len(data)-BLOCK_SIZE, (BATCH_SIZE,))
    x = torch.stack([data[i:i+BLOCK_SIZE] for i in ix])
    y = torch.stack([data[i+1:i+BLOCK_SIZE+1] for i in ix])
    return x,y

# estimate_loss over range to average out noise
# The @torch.nograd() decorator in Python is used to indicate that the function 
# should not track or modify the gradients of any of the tensors it uses. 
# This is useful when you want to perform an operation on a tensor without 
# having to worry about the gradients. For example, if you are performing 
# a calculation in which the gradients are not relevant, you can use 
# @torch.nograd() to make sure that the gradient calculations are not performed.
@torch.no_grad()
def estimate_loss():
# TODO : Is there a difference whether we use implicit (referenced) model, or
# can we pass in the model as a parameter (copy)?
#def estimate_loss(model):
    out = {}
    # does model.eval() and model.train() need a reference to the model?, or can
    # they take a copy of the model as a parameter?
    model.eval()
    for split in ['train', VALIDATION_SPLIT]:
        losses = torch.zeros(EVAL_ITERS)
        for k in range(EVAL_ITERS):
            X,Y = get_batch(split)
            logits, loss = model(X, Y)
            losses[k] = loss.item()
        out[split] = losses.mean()
    model.train()
    return out

# super simple bigram model
class BigramLanguageModel(nn.Module):

    def __init__(self, vocab_size):
        # super().init() in Python is a special method used to call the parent 
        # class’s init() method. This is useful when you want to extend the 
        # functionality of the parent class by adding additional attributes or 
        # methods. For example, if you have a class that inherits from another 
        # class, you can use super().init() to call the parent class’s init() 
        # and set any additional attributes or methods that the parent class 
        # does not have.
        super().__init__()

        # each token directly reads off the logits for the next token from a 
        # lookup table
        self.token_embedding_table = nn.Embedding(vocab_size, vocab_size)

    def forward(self, idx, targets=None):
        #idx and targets are both (B(atch), T(ime)) tensors of integers
        logits = self.token_embedding_table(idx) # (B, T, C)

        if targets is None:
            loss = None
        else:
            B,T,C = logits.shape
            # TODO : what is the difference between view and reshape?
            logits = logits.view(B*T, C)
            targets = targets.view(B*T)
            # ^^^ view/reshaping is to match cross entropy loss input 
            # requirements
            loss = F.cross_entropy(logits, targets)

        return logits, loss

    def generate(self, idx, max_new_tokens):
        # idx is a (B,T) array of indices in the current context
        for _ in range(max_new_tokens):
            # get the predictions
            logits, loss = self(idx)
            # focus only on the last time step (time step = prediction?)
            # NOTE : -1 in the T dimension gets us the last time step
            logits = logits[:, -1, :] # becomes (B, C) tensor
            # apply softmax to get probabilities
            # TODO : understand what's happening here kk
            probs = F.softmax(logits, dim=1) # (B, C)
            # sample from the distribution
            # TODO : understand what's happening here too
            idx_next = torch.multinomial(probs, num_samples=1) # (B, 1)
            # append sampled index to the running sequence
            idx = torch.cat((idx, idx_next), dim=1) # (B, T+1)
        return idx

model = BigramLanguageModel(vocab_size)
# move to device and run there if available
m = model.to(DEVICE)

# create a pytorch optimizer
# TODO : what is AdamW? And why is it better than SGD?
# Answer : the main difference between SGD and AdamW is :
# - AdamW incorporates weight decay into the optimizer step size, weight decay is
#   is a regularization technique that helps prevent overfitting by ensuring that
#   weights remain close to their initial values
# - AdamW also incorporates momentum into the optimizer step size -- TODO: what is momentum?
# - SGD is STOCHASTIC, meaning that it takes small steps towards the minimum of 
#   the loss function, whereas AdamW is ADAPTIVE which takes larger steps AND 
#   adjusts step size according to the current state of the system
optimizer = torch.optim.AdamW(model.parameters(), lr=LEARNING_RATE)

for iter in range(MAX_ITERS):
    # every once in a while evaluate the loss on train and val sets
    if iter%EVAL_INTERVAL==0:
        losses = estimate_loss()
        print(f"step {iter} : train loss = {losses['train']:0.4f}, val loss {losses[VALIDATION_SPLIT]:0.4f}")

        # sample a batch of data
        xb, yb = get_batch('train')

        # evaluate the loss
        logits, loss = model(xb, yb)
        optimizer.zero_grad(set_to_none=True)
        loss.backward()
        optimizer.step()

# generate from the model
context = torch.zeros((1,1), dtype=torch.long)
print(decode(m.generate(context, max_new_tokens=500)[0].tolist()))




