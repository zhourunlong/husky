import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
import wandb
from datasets.mini_husky import MiniHuskyDataset
from models.transformer import EncoderOnlyTransformer, SimpleTransformer
from train import train_model, eval_model
from utils import set_seed, custom_collate, linear_warmup_decay

import os
import pdb

# Load the dataset
dataset = MiniHuskyDataset(
    n=100000,
    max_fns=100,
    error_rate=0.10,
    data_dir=os.path.join(os.environ["DATA_ROOT"], "mini-husky/basic")
)
vocab_size = len(dataset.vocab)
output_dims = dataset.get_output_dims()
output_dim = sum(output_dims)

# Split the dataset into train and validation sets
train_size = int(0.8 * len(dataset))  # 80% for training
val_size = len(dataset) - train_size  # 20% for validation
train_dataset, val_dataset = random_split(dataset, [train_size, val_size])

def run(config):
    global vocab_size

    set_seed(config["seed"])

    train_dataloader = DataLoader(
        train_dataset,
        batch_size=config["batch_size"],
        shuffle=True,
        collate_fn=custom_collate,
    )
    val_dataloader = DataLoader(
        val_dataset,
        batch_size=config["batch_size"],
        shuffle=False,
        collate_fn=custom_collate,
    )

    exp_name = f"transformer_{config['hidden_dim']}_{config['d_model']}_{config['num_heads']}_{config['num_layers']}_seed{config['seed']}_bs{config['batch_size']}"

    # Initialize wandb
    wandb.init(project="transformer-vs-rnn", name=exp_name, config=config)

    # Set device to CUDA if available
    device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
    print(f"Using device: {device}")

    # Create models
    model = EncoderOnlyTransformer(
        vocab_size=vocab_size,
        output_dim=output_dim,  # Updated to use output_dims from dataset
        num_layers=config["num_layers"],
        num_heads=config["num_heads"],
        d_model=config["d_model"],
        max_seq_len=4096,
    ).to(device)

    # Loss function and optimizers
    criterion = nn.CrossEntropyLoss(reduction="sum", ignore_index=-1)
    optimizer = optim.Adam(model.parameters(), lr=1)
    scheduler = torch.optim.lr_scheduler.LambdaLR(
        optimizer,
        lr_lambda=linear_warmup_decay(
            total_iters=config["num_epochs"] * len(train_dataloader),
            **config
        )
    )

    # Train the models
    print("Training Transformer...")
    best_model = train_model(
        model=model,
        train_dataloader=train_dataloader,
        val_dataloader=val_dataloader,
        criterion=criterion,
        optimizer=optimizer,
        scheduler=scheduler,
        num_epochs=config["num_epochs"],
        micro_batch_size=config["micro_batch_size"],
        device=device,  # Add this line
        output_dims=output_dims,  # Use the retrieved output_dims
        log_wandb=True,
    )

    wandb.finish()

for hidden_dim in [16, 32, 64]:
    for d_model in [16, 32]:
        for num_heads in [2, 4]:
            for num_layers in [2, 4, 8]:
                config = {
                    "seed": 42,
                    "batch_size": 512,
                    "micro_batch_size": 64,
                    "num_epochs": 4,
                    "min_lr": 1e-6,
                    "max_lr": 1e-4,
                    "warmup_iters": 990,
                    "hidden_dim": hidden_dim,
                    "d_model": d_model,
                    "num_heads": num_heads,
                    "num_layers": num_layers,
                }
                run(config)
