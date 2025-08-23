import json
import csv
from collections import defaultdict

# All your transaction data
txn_data = """2025-08-13 02:24:47.000000 UTC\t5QEKDdkcKHQ2cpGWA5LvmX9kjh3T24x71e9sXotyKMFkEaE3tnSkUwuHd2EvJsKAiz1XpGt2KJNR6pM8KzG2999j\tBurn\t{"ConfidentialTransferFeeInstructionFields":null,"ConfidentialTransferInstructionFields":null,"DefaultAccountStateInstructionFields":null,"GroupMemberPointerFields":null,"GroupPointerFields":null,"InterestBearingMintInstructionFields":null,"MetadataPointerFields":null,"TransferFeeInstructionFields":null,"TransferHookInstructionFields":null,"amount":1622683921.0,"authority_type":null,"close_authority":null,"decimals":null,"delegate":null,"extension_types":null,"freeze_authority":null,"m":null,"mint_authority":null,"name":null,"new_authority":null,"owner":null,"ui_amount":null}\t{"ApplyPendingBalance":null,"ApproveAccount":null,"ConfigureAccount":null,"Deposit":null,"Disable":null,"DisableConfidentialCredits":null,"DisableHarvestToMint":null,"DisableNonConfidentialCredits":null,"EmptyAccount":null,"Enable":null,"EnableConfidentialCredits":null,"EnableHarvestToMint":null,"EnableNonConfidentialCredits":null,"HarvestWithheldTokensToMint":null,"Initialize":null,"InitializeConfidentialTransferFeeConfig":null,"InitializeMint":null,"InitializeTransferFeeConfig":null,"SetTransferFee":null,"Transfer":null,"TransferCheckedWithFee":null,"TransferWithSplitProofs":null,"Update":null,"UpdateMint":null,"UpdateRate":null,"Withdraw":null,"WithdrawWithheldTokensFromAccounts":null,"WithdrawWithheldTokensFromMint":null,"account":"3SrTmuE4n5RFeBHF9cD31kDLM4iNsZH5niXy4mMVLDsm","accounts":null,"authority":null,"current_authority":null,"delegate":null,"destination":null,"funding_account":null,"ming_associated":null,"mint":"9PtoiZTQAXhcoVTwQ4NbaXJ2f1oGptpvSiQc8KZ5E6GK","mint_authority":null,"owner":"Ewb8XmZ4RbSAxnFY4P4XMtjz5EccYn12bnUF6T4SSvVq","payer":null,"rent_sysvar":null,"rest_sysvar":null,"signers":{"list":[]},"source":null,"system_program":null}
2025-08-13 02:24:32.000000 UTC\t2YgP6NSwJX9LkxQDokzrmHji1TbeBTc9ZYJAUns9Qw3NEQzfybiPT1qyHzRN2ehnT6WNgZqCFLPJeW6sbesosDKH\tBurnChecked\t{"ConfidentialTransferFeeInstructionFields":null,"ConfidentialTransferInstructionFields":null,"DefaultAccountStateInstructionFields":null,"GroupMemberPointerFields":null,"GroupPointerFields":null,"InterestBearingMintInstructionFields":null,"MetadataPointerFields":null,"TransferFeeInstructionFields":null,"TransferHookInstructionFields":null,"amount":1.0,"authority_type":null,"close_authority":null,"decimals":0,"delegate":null,"extension_types":null,"freeze_authority":null,"m":null,"mint_authority":null,"name":null,"new_authority":null,"owner":null,"ui_amount":null}\t{"ApplyPendingBalance":null,"ApproveAccount":null,"ConfigureAccount":null,"Deposit":null,"Disable":null,"DisableConfidentialCredits":null,"DisableHarvestToMint":null,"DisableNonConfidentialCredits":null,"EmptyAccount":null,"Enable":null,"EnableConfidentialCredits":null,"EnableHarvestToMint":null,"EnableNonConfidentialCredits":null,"HarvestWithheldTokensToMint":null,"Initialize":null,"InitializeConfidentialTransferFeeConfig":null,"InitializeMint":null,"InitializeTransferFeeConfig":null,"SetTransferFee":null,"Transfer":null,"TransferCheckedWithFee":null,"TransferWithSplitProofs":null,"Update":null,"UpdateMint":null,"UpdateRate":null,"Withdraw":null,"WithdrawWithheldTokensFromAccounts":null,"WithdrawWithheldTokensFromMint":null,"account":"Fj8qRo8n4r38p3Nhyqh63tLfdfp1riKB9aaUe4Tzwwvx","accounts":null,"authority":null,"current_authority":null,"delegate":null,"destination":null,"funding_account":null,"ming_associated":null,"mint":"8fJs1aeEaotbae2yAdUHzRm1D3LaxWJBEa8rFA7LzrD8","mint_authority":null,"owner":"HaN1GryJcetFQBixJaMPzKneDzfGyDV3s8rzLtWVGf1x","payer":null,"rent_sysvar":null,"rest_sysvar":null,"signers":{"list":[]},"source":null,"system_program":null}
2025-08-13 02:24:47.000000 UTC\t3p9RDWGNgGxNPzoJH786KB5PGQ4bUedCC8VgAvL7iTGoeBktQvjfcFo6vgbivArzUss5qrwA2vhh3mjzQjDGq9TK\tCloseAccount\t{"ConfidentialTransferFeeInstructionFields":null,"ConfidentialTransferInstructionFields":null,"DefaultAccountStateInstructionFields":null,"GroupMemberPointerFields":null,"GroupPointerFields":null,"InterestBearingMintInstructionFields":null,"MetadataPointerFields":null,"TransferFeeInstructionFields":null,"TransferHookInstructionFields":null,"amount":null,"authority_type":null,"close_authority":null,"decimals":null,"delegate":null,"extension_types":null,"freeze_authority":null,"m":null,"mint_authority":null,"name":null,"new_authority":null,"owner":null,"ui_amount":null}\t{"ApplyPendingBalance":null,"ApproveAccount":null,"ConfigureAccount":null,"Deposit":null,"Disable":null,"DisableConfidentialCredits":null,"DisableHarvestToMint":null,"DisableNonConfidentialCredits":null,"EmptyAccount":null,"Enable":null,"EnableConfidentialCredits":null,"EnableHarvestToMint":null,"EnableNonConfidentialCredits":null,"HarvestWithheldTokensToMint":null,"Initialize":null,"InitializeConfidentialTransferFeeConfig":null,"InitializeMint":null,"InitializeTransferFeeConfig":null,"SetTransferFee":null,"Transfer":null,"TransferCheckedWithFee":null,"TransferWithSplitProofs":null,"Update":null,"UpdateMint":null,"UpdateRate":null,"Withdraw":null,"WithdrawWithheldTokensFromAccounts":null,"WithdrawWithheldTokensFromMint":null,"account":"HzwjPcpgh9i1eVYAy27FPRQEU5x3n3CuVLKRVXKtWQRo","accounts":null,"authority":null,"current_authority":null,"delegate":null,"destination":"73deVuYLUUU4ez3Z4LbX842oFL2jcpbTWWJke5nFuR7D","funding_account":null,"ming_associated":null,"mint":null,"mint_authority":null,"owner":"EUT2MFBXtT4ZdwGH59u3bZhTQ2NbXWeeQr6nfM8o84mS","payer":null,"rent_sysvar":null,"rest_sysvar":null,"signers":{"list":[]},"source":null,"system_program":null}
2025-08-13 02:22:12.000000 UTC\t2wpVHHmNJHnUy5bhrZWiaqhNSVL57WjZM2akZuffkLy6Ze9VjR14RgfgQkcRHCYGWE4piFykEBLwT6wxD6T8Kjdv\tFreezeAccount\t{"ConfidentialTransferFeeInstructionFields":null,"ConfidentialTransferInstructionFields":null,"DefaultAccountStateInstructionFields":null,"GroupMemberPointerFields":null,"GroupPointerFields":null,"InterestBearingMintInstructionFields":null,"MetadataPointerFields":null,"TransferFeeInstructionFields":null,"TransferHookInstructionFields":null,"amount":null,"authority_type":null,"close_authority":null,"decimals":null,"delegate":null,"extension_types":null,"freeze_authority":null,"m":null,"mint_authority":null,"name":null,"new_authority":null,"owner":null,"ui_amount":null}\t{"ApplyPendingBalance":null,"ApproveAccount":null,"ConfigureAccount":null,"Deposit":null,"Disable":null,"DisableConfidentialCredits":null,"DisableHarvestToMint":null,"DisableNonConfidentialCredits":null,"EmptyAccount":null,"Enable":null,"EnableConfidentialCredits":null,"EnableHarvestToMint":null,"EnableNonConfidentialCredits":null,"HarvestWithheldTokensToMint":null,"Initialize":null,"InitializeConfidentialTransferFeeConfig":null,"InitializeMint":null,"InitializeTransferFeeConfig":null,"SetTransferFee":null,"Transfer":null,"TransferCheckedWithFee":null,"TransferWithSplitProofs":null,"Update":null,"UpdateMint":null,"UpdateRate":null,"Withdraw":null,"WithdrawWithheldTokensFromAccounts":null,"WithdrawWithheldTokensFromMint":null,"account":"AhpGfoYoFVdJmg2EtrZbuxQke4j6dxLNdxMKUhYcVrkX","accounts":null,"authority":"6w8zRqduVqMngQkzsoFBSsJyaTPZ3BbYPugXdu5SXRP","current_authority":null,"delegate":null,"destination":null,"funding_account":null,"ming_associated":null,"mint":"XRPGDPJiuivMWmCWouAYReXuxtw5qQhG3p5oJFVuQov","mint_authority":null,"owner":null,"payer":null,"rent_sysvar":null,"rest_sysvar":null,"signers":{"list":[]},"source":null,"system_program":null}
2025-08-13 02:24:47.000000 UTC\t4bWAdod2GBizCX4aEdxmFs4K2VhVFs4XXvUjyqrJ1KcTpZMPhSiFQfn7QEZ8SnTboYSwAwHfk5e64XwqUbqcdiGB\tInitializeMint2\t{"ConfidentialTransferFeeInstructionFields":null,"ConfidentialTransferInstructionFields":null,"DefaultAccountStateInstructionFields":null,"GroupMemberPointerFields":null,"GroupPointerFields":null,"InterestBearingMintInstructionFields":null,"MetadataPointerFields":null,"TransferFeeInstructionFields":null,"TransferHookInstructionFields":null,"amount":null,"authority_type":null,"close_authority":null,"decimals":0,"delegate":null,"extension_types":null,"freeze_authority":"JACuhnN1fnmRTptZSDACNc3dbGB8EgV8Hu19q6gpwQk2","m":null,"mint_authority":"JACuhnN1fnmRTptZSDACNc3dbGB8EgV8Hu19q6gpwQk2","name":null,"new_authority":null,"owner":null,"ui_amount":null}\t{"ApplyPendingBalance":null,"ApproveAccount":null,"ConfigureAccount":null,"Deposit":null,"Disable":null,"DisableConfidentialCredits":null,"DisableHarvestToMint":null,"DisableNonConfidentialCredits":null,"EmptyAccount":null,"Enable":null,"EnableConfidentialCredits":null,"EnableHarvestToMint":null,"EnableNonConfidentialCredits":null,"HarvestWithheldTokensToMint":null,"Initialize":null,"InitializeConfidentialTransferFeeConfig":null,"InitializeMint":null,"InitializeTransferFeeConfig":null,"SetTransferFee":null,"Transfer":null,"TransferCheckedWithFee":null,"TransferWithSplitProofs":null,"Update":null,"UpdateMint":null,"UpdateRate":null,"Withdraw":null,"WithdrawWithheldTokensFromAccounts":null,"WithdrawWithheldTokensFromMint":null,"account":null,"accounts":null,"authority":null,"current_authority":null,"delegate":null,"destination":null,"funding_account":null,"ming_associated":null,"mint":"9h7bXmrxrJAaF43r8Y3NVpVFxXzePEXn6sY4dPn476bS","mint_authority":null,"owner":null,"payer":null,"rent_sysvar":null,"rest_sysvar":null,"signers":null,"source":null,"system_program":null}"""

# Continue with rest of the data (all 43 instructions)...

# Add ALL the rest of your transaction data here...
txn_data += """
2025-08-13 02:24:47.000000 UTC\t4bWAdod2GBizCX4aEdxmFs4K2VhVFs4XXvUjyqrJ1KcTpZMPhSiFQfn7QEZ8SnTboYSwAwHfk5e64XwqUbqcdiGB\tGetAccountDataSize\t{"ConfidentialTransferFeeInstructionFields":null,"ConfidentialTransferInstructionFields":null,"DefaultAccountStateInstructionFields":null,"GroupMemberPointerFields":null,"GroupPointerFields":null,"InterestBearingMintInstructionFields":null,"MetadataPointerFields":null,"TransferFeeInstructionFields":null,"TransferHookInstructionFields":null,"amount":null,"authority_type":null,"close_authority":null,"decimals":null,"delegate":null,"extension_types":{"list":[]},"freeze_authority":null,"m":null,"mint_authority":null,"name":null,"new_authority":null,"owner":null,"ui_amount":null}\t{"ApplyPendingBalance":null,"ApproveAccount":null,"ConfigureAccount":null,"Deposit":null,"Disable":null,"DisableConfidentialCredits":null,"DisableHarvestToMint":null,"DisableNonConfidentialCredits":null,"EmptyAccount":null,"Enable":null,"EnableConfidentialCredits":null,"EnableHarvestToMint":null,"EnableNonConfidentialCredits":null,"HarvestWithheldTokensToMint":null,"Initialize":null,"InitializeConfidentialTransferFeeConfig":null,"InitializeMint":null,"InitializeTransferFeeConfig":null,"SetTransferFee":null,"Transfer":null,"TransferCheckedWithFee":null,"TransferWithSplitProofs":null,"Update":null,"UpdateMint":null,"UpdateRate":null,"Withdraw":null,"WithdrawWithheldTokensFromAccounts":null,"WithdrawWithheldTokensFromMint":null,"account":null,"accounts":null,"authority":null,"current_authority":null,"delegate":null,"destination":null,"funding_account":null,"ming_associated":null,"mint":"9h7bXmrxrJAaF43r8Y3NVpVFxXzePEXn6sY4dPn476bS","mint_authority":null,"owner":null,"payer":null,"rent_sysvar":null,"rest_sysvar":null,"signers":null,"source":null,"system_program":null}
2025-08-13 02:24:47.000000 UTC\t4bWAdod2GBizCX4aEdxmFs4K2VhVFs4XXvUjyqrJ1KcTpZMPhSiFQfn7QEZ8SnTboYSwAwHfk5e64XwqUbqcdiGB\tInitializeImmutableOwner\t{"ConfidentialTransferFeeInstructionFields":null,"ConfidentialTransferInstructionFields":null,"DefaultAccountStateInstructionFields":null,"GroupMemberPointerFields":null,"GroupPointerFields":null,"InterestBearingMintInstructionFields":null,"MetadataPointerFields":null,"TransferFeeInstructionFields":null,"TransferHookInstructionFields":null,"amount":null,"authority_type":null,"close_authority":null,"decimals":null,"delegate":null,"extension_types":null,"freeze_authority":null,"m":null,"mint_authority":null,"name":null,"new_authority":null,"owner":null,"ui_amount":null}\t{"ApplyPendingBalance":null,"ApproveAccount":null,"ConfigureAccount":null,"Deposit":null,"Disable":null,"DisableConfidentialCredits":null,"DisableHarvestToMint":null,"DisableNonConfidentialCredits":null,"EmptyAccount":null,"Enable":null,"EnableConfidentialCredits":null,"EnableHarvestToMint":null,"EnableNonConfidentialCredits":null,"HarvestWithheldTokensToMint":null,"Initialize":null,"InitializeConfidentialTransferFeeConfig":null,"InitializeMint":null,"InitializeTransferFeeConfig":null,"SetTransferFee":null,"Transfer":null,"TransferCheckedWithFee":null,"TransferWithSplitProofs":null,"Update":null,"UpdateMint":null,"UpdateRate":null,"Withdraw":null,"WithdrawWithheldTokensFromAccounts":null,"WithdrawWithheldTokensFromMint":null,"account":"J36HRVo3Tj2ZrGytAZCcZ8HA8FiGcSvtrynV25hbrc1c","accounts":null,"authority":null,"current_authority":null,"delegate":null,"destination":null,"funding_account":null,"ming_associated":null,"mint":null,"mint_authority":null,"owner":null,"payer":null,"rent_sysvar":null,"rest_sysvar":null,"signers":null,"source":null,"system_program":null}
2025-08-13 02:22:12.000000 UTC\t2wpVHHmNJHnUy5bhrZWiaqhNSVL57WjZM2akZuffkLy6Ze9VjR14RgfgQkcRHCYGWE4piFykEBLwT6wxD6T8Kjdv\tThawAccount\t{"ConfidentialTransferFeeInstructionFields":null,"ConfidentialTransferInstructionFields":null,"DefaultAccountStateInstructionFields":null,"GroupMemberPointerFields":null,"GroupPointerFields":null,"InterestBearingMintInstructionFields":null,"MetadataPointerFields":null,"TransferFeeInstructionFields":null,"TransferHookInstructionFields":null,"amount":null,"authority_type":null,"close_authority":null,"decimals":null,"delegate":null,"extension_types":null,"freeze_authority":null,"m":null,"mint_authority":null,"name":null,"new_authority":null,"owner":null,"ui_amount":null}\t{"ApplyPendingBalance":null,"ApproveAccount":null,"ConfigureAccount":null,"Deposit":null,"Disable":null,"DisableConfidentialCredits":null,"DisableHarvestToMint":null,"DisableNonConfidentialCredits":null,"EmptyAccount":null,"Enable":null,"EnableConfidentialCredits":null,"EnableHarvestToMint":null,"EnableNonConfidentialCredits":null,"HarvestWithheldTokensToMint":null,"Initialize":null,"InitializeConfidentialTransferFeeConfig":null,"InitializeMint":null,"InitializeTransferFeeConfig":null,"SetTransferFee":null,"Transfer":null,"TransferCheckedWithFee":null,"TransferWithSplitProofs":null,"Update":null,"UpdateMint":null,"UpdateRate":null,"Withdraw":null,"WithdrawWithheldTokensFromAccounts":null,"WithdrawWithheldTokensFromMint":null,"account":"AhpGfoYoFVdJmg2EtrZbuxQke4j6dxLNdxMKUhYcVrkX","accounts":null,"authority":"6w8zRqduVqMngQkzsoFBSsJyaTPZ3BbYPugXdu5SXRP","current_authority":null,"delegate":null,"destination":null,"funding_account":null,"ming_associated":null,"mint":"XRPGDPJiuivMWmCWouAYReXuxtw5qQhG3p5oJFVuQov","mint_authority":null,"owner":null,"payer":null,"rent_sysvar":null,"rest_sysvar":null,"signers":{"list":[]},"source":null,"system_program":null}
2025-08-13 02:23:58.000000 UTC\t2reUs4oYTfDeSZf9CsJXDmC2BnT9gLyc9VRnsvAoeLyfWktyZtoT25BWnEW6wDvmjDQEkYYxRJYCwvnFFCXiTMk\tMintToChecked\t{"ConfidentialTransferFeeInstructionFields":null,"ConfidentialTransferInstructionFields":null,"DefaultAccountStateInstructionFields":null,"GroupMemberPointerFields":null,"GroupPointerFields":null,"InterestBearingMintInstructionFields":null,"MetadataPointerFields":null,"TransferFeeInstructionFields":null,"TransferHookInstructionFields":null,"amount":1e+15,"authority_type":null,"close_authority":null,"decimals":6,"delegate":null,"extension_types":null,"freeze_authority":null,"m":null,"mint_authority":null,"name":null,"new_authority":null,"owner":null,"ui_amount":null}\t{"ApplyPendingBalance":null,"ApproveAccount":null,"ConfigureAccount":null,"Deposit":null,"Disable":null,"DisableConfidentialCredits":null,"DisableHarvestToMint":null,"DisableNonConfidentialCredits":null,"EmptyAccount":null,"Enable":null,"EnableConfidentialCredits":null,"EnableHarvestToMint":null,"EnableNonConfidentialCredits":null,"HarvestWithheldTokensToMint":null,"Initialize":null,"InitializeConfidentialTransferFeeConfig":null,"InitializeMint":null,"InitializeTransferFeeConfig":null,"SetTransferFee":null,"Transfer":null,"TransferCheckedWithFee":null,"TransferWithSplitProofs":null,"Update":null,"UpdateMint":null,"UpdateRate":null,"Withdraw":null,"WithdrawWithheldTokensFromAccounts":null,"WithdrawWithheldTokensFromMint":null,"account":"4eTb1AnXiUaLVmKXYMuLg4rcwFiPSpBrw7Cj5QogvNKD","accounts":null,"authority":null,"current_authority":null,"delegate":null,"destination":null,"funding_account":null,"ming_associated":null,"mint":"88TpMvuKVP9tLZwHyv5Vk22TxVVLZAT6ropwjab7cBM2","mint_authority":"Ewb8XmZ4RbSAxnFY4P4XMtjz5EccYn12bnUF6T4SSvVq","owner":null,"payer":null,"rent_sysvar":null,"rest_sysvar":null,"signers":{"list":[]},"source":null,"system_program":null}"""

def extract_canonical_name(instruction_type, args_json_str):
    try:
        args = json.loads(args_json_str)
        if instruction_type.endswith("Extension"):
            name = args.get("name")
            if name:
                base_name = name[0].lower() + name[1:] if name else name
                return base_name + instruction_type.replace("Extension", "")
        return instruction_type[0].lower() + instruction_type[1:] if instruction_type else instruction_type
    except:
        return instruction_type[0].lower() + instruction_type[1:] if instruction_type else instruction_type

def count_non_null_args(args_json_str):
    try:
        args = json.loads(args_json_str)
        count = 0
        for key, value in args.items():
            if value is not None:
                if isinstance(value, dict):
                    count += sum(1 for v in value.values() if v is not None)
                elif isinstance(value, list):
                    count += len(value)
                else:
                    count += 1
        return count
    except:
        return 0

def count_non_null_accounts(accounts_json_str):
    try:
        accounts = json.loads(accounts_json_str)
        count = 0
        for key, value in accounts.items():
            if value is not None:
                if key in ["signers", "accounts"] and isinstance(value, dict) and "list" in value:
                    count += len(value["list"]) if value["list"] else 0
                elif isinstance(value, str):
                    count += 1
        return count
    except:
        return 0

# Load IDL lookup
idl_instructions = {}
try:
    with open('/Users/admin/solana-rust-decoders/token_2022_instructions.csv', 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            idl_instructions[row['instruction_name']] = {
                'accounts_count': int(row['accounts_count']),
                'args_count': int(row['args_count'])
            }
except:
    pass

# Process data
instruction_stats = defaultdict(lambda: {'count': 0, 'args_total': 0, 'accounts_total': 0})

lines = txn_data.strip().split('\n')
for line in lines:
    if line.strip():
        parts = line.split('\t')
        if len(parts) >= 5:
            instruction_type = parts[2]
            args_json = parts[3]
            accounts_json = parts[4]
            
            canonical_name = extract_canonical_name(instruction_type, args_json)
            instruction_stats[canonical_name]['count'] += 1
            instruction_stats[canonical_name]['args_total'] += count_non_null_args(args_json)
            instruction_stats[canonical_name]['accounts_total'] += count_non_null_accounts(accounts_json)

# Write CSV
output_file = '/Users/admin/Desktop/all_43_instructions_analysis.csv'
with open(output_file, 'w', newline='') as csvfile:
    fieldnames = ['instruction_name', 'occurrence_count', 'args_non_null_count', 'accounts_non_null_count', 'idl_accounts_count', 'idl_args_count']
    writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
    writer.writeheader()
    
    for name, stats in sorted(instruction_stats.items()):
        idl_data = idl_instructions.get(name, {'accounts_count': 0, 'args_count': 0})
        writer.writerow({
            'instruction_name': name,
            'occurrence_count': stats['count'],
            'args_non_null_count': stats['args_total'],
            'accounts_non_null_count': stats['accounts_total'],
            'idl_accounts_count': idl_data['accounts_count'],
            'idl_args_count': idl_data['args_count']
        })

print(f"CSV saved to {output_file}")
print(f"Processed {len(instruction_stats)} distinct instructions")
