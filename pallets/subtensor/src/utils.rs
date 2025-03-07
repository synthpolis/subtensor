use super::*;
use crate::system::{ensure_root, ensure_signed_or_root};
use sp_core::U256;

impl<T: Config> Pallet<T> {
    pub fn ensure_subnet_owner_or_root(
        o: T::RuntimeOrigin,
        netuid: u16,
    ) -> Result<(), DispatchError> {
        let coldkey = ensure_signed_or_root(o);
        match coldkey {
            Ok(Some(who)) if SubnetOwner::<T>::get(netuid) == who => Ok(()),
            Ok(Some(_)) => Err(DispatchError::BadOrigin),
            Ok(None) => Ok(()),
            Err(x) => Err(x.into()),
        }
    }

    // ========================
    // ==== Global Setters ====
    // ========================
    pub fn set_tempo(netuid: u16, tempo: u16) {
        Tempo::<T>::insert(netuid, tempo);
        Self::deposit_event(Event::TempoSet(netuid, tempo));
    }
    pub fn set_last_adjustment_block(netuid: u16, last_adjustment_block: u64) {
        LastAdjustmentBlock::<T>::insert(netuid, last_adjustment_block);
    }
    pub fn set_blocks_since_last_step(netuid: u16, blocks_since_last_step: u64) {
        BlocksSinceLastStep::<T>::insert(netuid, blocks_since_last_step);
    }
    pub fn set_registrations_this_block(netuid: u16, registrations_this_block: u16) {
        RegistrationsThisBlock::<T>::insert(netuid, registrations_this_block);
    }
    pub fn set_last_mechanism_step_block(netuid: u16, last_mechanism_step_block: u64) {
        LastMechansimStepBlock::<T>::insert(netuid, last_mechanism_step_block);
    }
    pub fn set_registrations_this_interval(netuid: u16, registrations_this_interval: u16) {
        RegistrationsThisInterval::<T>::insert(netuid, registrations_this_interval);
    }
    pub fn set_pow_registrations_this_interval(netuid: u16, pow_registrations_this_interval: u16) {
        POWRegistrationsThisInterval::<T>::insert(netuid, pow_registrations_this_interval);
    }
    pub fn set_burn_registrations_this_interval(
        netuid: u16,
        burn_registrations_this_interval: u16,
    ) {
        BurnRegistrationsThisInterval::<T>::insert(netuid, burn_registrations_this_interval);
    }

    // ========================
    // ==== Global Getters ====
    // ========================
    pub fn get_total_issuance() -> u64 {
        TotalIssuance::<T>::get()
    }
    pub fn get_current_block_as_u64() -> u64 {
        TryInto::try_into(<frame_system::Pallet<T>>::block_number())
            .ok()
            .expect("blockchain will not exceed 2^64 blocks; QED.")
    }

    // ==============================
    // ==== YumaConsensus params ====
    // ==============================
    pub fn get_rank(netuid: u16) -> Vec<u16> {
        Rank::<T>::get(netuid)
    }
    pub fn get_trust(netuid: u16) -> Vec<u16> {
        Trust::<T>::get(netuid)
    }
    pub fn get_active(netuid: u16) -> Vec<bool> {
        Active::<T>::get(netuid)
    }
    pub fn get_emission(netuid: u16) -> Vec<u64> {
        Emission::<T>::get(netuid)
    }
    pub fn get_consensus(netuid: u16) -> Vec<u16> {
        Consensus::<T>::get(netuid)
    }
    pub fn get_incentive(netuid: u16) -> Vec<u16> {
        Incentive::<T>::get(netuid)
    }
    pub fn get_dividends(netuid: u16) -> Vec<u16> {
        Dividends::<T>::get(netuid)
    }
    pub fn get_last_update(netuid: u16) -> Vec<u64> {
        LastUpdate::<T>::get(netuid)
    }
    pub fn get_pruning_score(netuid: u16) -> Vec<u16> {
        PruningScores::<T>::get(netuid)
    }
    pub fn get_validator_trust(netuid: u16) -> Vec<u16> {
        ValidatorTrust::<T>::get(netuid)
    }
    pub fn get_validator_permit(netuid: u16) -> Vec<bool> {
        ValidatorPermit::<T>::get(netuid)
    }

    // ==================================
    // ==== YumaConsensus UID params ====
    // ==================================
    pub fn set_last_update_for_uid(netuid: u16, uid: u16, last_update: u64) {
        let mut updated_last_update_vec = Self::get_last_update(netuid);
        if (uid as usize) < updated_last_update_vec.len() {
            updated_last_update_vec[uid as usize] = last_update;
            LastUpdate::<T>::insert(netuid, updated_last_update_vec);
        }
    }
    pub fn set_active_for_uid(netuid: u16, uid: u16, active: bool) {
        let mut updated_active_vec = Self::get_active(netuid);
        if (uid as usize) < updated_active_vec.len() {
            updated_active_vec[uid as usize] = active;
            Active::<T>::insert(netuid, updated_active_vec);
        }
    }
    pub fn set_pruning_score_for_uid(netuid: u16, uid: u16, pruning_score: u16) {
        log::info!("netuid = {:?}", netuid);
        log::info!(
            "SubnetworkN::<T>::get( netuid ) = {:?}",
            SubnetworkN::<T>::get(netuid)
        );
        log::info!("uid = {:?}", uid);
        assert!(uid < SubnetworkN::<T>::get(netuid));
        PruningScores::<T>::mutate(netuid, |v| v[uid as usize] = pruning_score);
    }
    pub fn set_validator_permit_for_uid(netuid: u16, uid: u16, validator_permit: bool) {
        let mut updated_validator_permit = Self::get_validator_permit(netuid);
        if (uid as usize) < updated_validator_permit.len() {
            updated_validator_permit[uid as usize] = validator_permit;
            ValidatorPermit::<T>::insert(netuid, updated_validator_permit);
        }
    }
    pub fn set_weights_min_stake(min_stake: u64) {
        WeightsMinStake::<T>::put(min_stake);
        Self::deposit_event(Event::WeightsMinStake(min_stake));
    }
    pub fn set_target_stakes_per_interval(target_stakes_per_interval: u64) {
        TargetStakesPerInterval::<T>::set(target_stakes_per_interval)
    }
    pub fn set_stakes_this_interval_for_hotkey(hotkey: &T::AccountId, stakes_this_interval: u64, last_staked_block_number: u64) {
        TotalHotkeyStakesThisInterval::<T>::insert(hotkey, (stakes_this_interval, last_staked_block_number));
    }
    pub fn set_stake_interval(block: u64) {
        StakeInterval::<T>::set(block);
    }
    pub fn get_rank_for_uid(netuid: u16, uid: u16) -> u16 {
        let vec = Rank::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            0
        }
    }
    pub fn get_trust_for_uid(netuid: u16, uid: u16) -> u16 {
        let vec = Trust::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            0
        }
    }
    pub fn get_emission_for_uid(netuid: u16, uid: u16) -> u64 {
        let vec = Emission::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            0
        }
    }
    pub fn get_active_for_uid(netuid: u16, uid: u16) -> bool {
        let vec = Active::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            false
        }
    }
    pub fn get_consensus_for_uid(netuid: u16, uid: u16) -> u16 {
        let vec = Consensus::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            0
        }
    }
    pub fn get_incentive_for_uid(netuid: u16, uid: u16) -> u16 {
        let vec = Incentive::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            0
        }
    }
    pub fn get_dividends_for_uid(netuid: u16, uid: u16) -> u16 {
        let vec = Dividends::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            0
        }
    }
    pub fn get_last_update_for_uid(netuid: u16, uid: u16) -> u64 {
        let vec = LastUpdate::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            0
        }
    }
    pub fn get_pruning_score_for_uid(netuid: u16, uid: u16) -> u16 {
        let vec = PruningScores::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            u16::MAX
        }
    }
    pub fn get_validator_trust_for_uid(netuid: u16, uid: u16) -> u16 {
        let vec = ValidatorTrust::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            0
        }
    }
    pub fn get_validator_permit_for_uid(netuid: u16, uid: u16) -> bool {
        let vec = ValidatorPermit::<T>::get(netuid);
        if (uid as usize) < vec.len() {
            vec[uid as usize]
        } else {
            false
        }
    }
    pub fn get_weights_min_stake() -> u64 {
        WeightsMinStake::<T>::get()
    }

    // ============================
    // ==== Subnetwork Getters ====
    // ============================
    pub fn get_tempo(netuid: u16) -> u16 {
        Tempo::<T>::get(netuid)
    }
    pub fn get_emission_value(netuid: u16) -> u64 {
        EmissionValues::<T>::get(netuid)
    }
    pub fn get_pending_emission(netuid: u16) -> u64 {
        PendingEmission::<T>::get(netuid)
    }
    pub fn get_last_adjustment_block(netuid: u16) -> u64 {
        LastAdjustmentBlock::<T>::get(netuid)
    }
    pub fn get_blocks_since_last_step(netuid: u16) -> u64 {
        BlocksSinceLastStep::<T>::get(netuid)
    }
    pub fn get_difficulty(netuid: u16) -> U256 {
        U256::from(Self::get_difficulty_as_u64(netuid))
    }
    pub fn get_registrations_this_block(netuid: u16) -> u16 {
        RegistrationsThisBlock::<T>::get(netuid)
    }
    pub fn get_last_mechanism_step_block(netuid: u16) -> u64 {
        LastMechansimStepBlock::<T>::get(netuid)
    }
    pub fn get_registrations_this_interval(netuid: u16) -> u16 {
        RegistrationsThisInterval::<T>::get(netuid)
    }
    pub fn get_pow_registrations_this_interval(netuid: u16) -> u16 {
        POWRegistrationsThisInterval::<T>::get(netuid)
    }
    pub fn get_burn_registrations_this_interval(netuid: u16) -> u16 {
        BurnRegistrationsThisInterval::<T>::get(netuid)
    }
    pub fn get_neuron_block_at_registration(netuid: u16, neuron_uid: u16) -> u64 {
        BlockAtRegistration::<T>::get(netuid, neuron_uid)
    }

    // ========================
    // ==== Rate Limiting =====
    // ========================
    pub fn set_last_tx_block(key: &T::AccountId, block: u64) {
        LastTxBlock::<T>::insert(key, block)
    }
    pub fn get_last_tx_block(key: &T::AccountId) -> u64 {
        LastTxBlock::<T>::get(key)
    }
    pub fn exceeds_tx_rate_limit(prev_tx_block: u64, current_block: u64) -> bool {
        let rate_limit: u64 = Self::get_tx_rate_limit();
        if rate_limit == 0 || prev_tx_block == 0 {
            return false;
        }

        current_block - prev_tx_block <= rate_limit
    }

    // ========================
    // === Token Management ===
    // ========================
    pub fn burn_tokens(amount: u64) {
        TotalIssuance::<T>::put(TotalIssuance::<T>::get().saturating_sub(amount));
    }
    pub fn coinbase(amount: u64 ){
        TotalIssuance::<T>::put(TotalIssuance::<T>::get().saturating_add(amount));
    }
    pub fn get_default_take() -> u16 {
        DefaultTake::<T>::get()
    }
    pub fn set_default_take(default_take: u16) {
        DefaultTake::<T>::put(default_take);
        Self::deposit_event(Event::DefaultTakeSet(default_take));
    }

    pub fn set_subnet_locked_balance(netuid: u16, amount: u64) {
        SubnetLocked::<T>::insert(netuid, amount);
    }

    pub fn get_subnet_locked_balance(netuid: u16) -> u64 {
        SubnetLocked::<T>::get(netuid)
    }

    // ========================
    // ========= Sudo =========
    // ========================

    // Configure tx rate limiting
    pub fn get_tx_rate_limit() -> u64 {
        TxRateLimit::<T>::get()
    }
    pub fn set_tx_rate_limit(tx_rate_limit: u64) {
        TxRateLimit::<T>::put(tx_rate_limit);
        Self::deposit_event(Event::TxRateLimitSet(tx_rate_limit));
    }

    pub fn get_serving_rate_limit(netuid: u16) -> u64 {
        ServingRateLimit::<T>::get(netuid)
    }
    pub fn set_serving_rate_limit(netuid: u16, serving_rate_limit: u64) {
        ServingRateLimit::<T>::insert(netuid, serving_rate_limit);
        Self::deposit_event(Event::ServingRateLimitSet(netuid, serving_rate_limit));
    }

    pub fn get_min_difficulty(netuid: u16) -> u64 {
        MinDifficulty::<T>::get(netuid)
    }
    pub fn set_min_difficulty(netuid: u16, min_difficulty: u64) {
        MinDifficulty::<T>::insert(netuid, min_difficulty);
        Self::deposit_event(Event::MinDifficultySet(netuid, min_difficulty));
    }

    pub fn get_max_difficulty(netuid: u16) -> u64 {
        MaxDifficulty::<T>::get(netuid)
    }
    pub fn set_max_difficulty(netuid: u16, max_difficulty: u64) {
        MaxDifficulty::<T>::insert(netuid, max_difficulty);
        Self::deposit_event(Event::MaxDifficultySet(netuid, max_difficulty));
    }

    pub fn get_weights_version_key(netuid: u16) -> u64 {
        WeightsVersionKey::<T>::get(netuid)
    }
    pub fn set_weights_version_key(netuid: u16, weights_version_key: u64) {
        WeightsVersionKey::<T>::insert(netuid, weights_version_key);
        Self::deposit_event(Event::WeightsVersionKeySet(netuid, weights_version_key));
    }

    pub fn get_weights_set_rate_limit(netuid: u16) -> u64 {
        WeightsSetRateLimit::<T>::get(netuid)
    }
    pub fn set_weights_set_rate_limit(netuid: u16, weights_set_rate_limit: u64) {
        WeightsSetRateLimit::<T>::insert(netuid, weights_set_rate_limit);
        Self::deposit_event(Event::WeightsSetRateLimitSet(
            netuid,
            weights_set_rate_limit,
        ));
    }

    pub fn get_adjustment_interval(netuid: u16) -> u16 {
        AdjustmentInterval::<T>::get(netuid)
    }
    pub fn set_adjustment_interval(netuid: u16, adjustment_interval: u16) {
        AdjustmentInterval::<T>::insert(netuid, adjustment_interval);
        Self::deposit_event(Event::AdjustmentIntervalSet(netuid, adjustment_interval));
    }

    pub fn get_adjustment_alpha(netuid: u16) -> u64 {
        AdjustmentAlpha::<T>::get(netuid)
    }
    pub fn set_adjustment_alpha(netuid: u16, adjustment_alpha: u64) {
        AdjustmentAlpha::<T>::insert(netuid, adjustment_alpha);
        Self::deposit_event(Event::AdjustmentAlphaSet(netuid, adjustment_alpha));
    }

    pub fn get_validator_prune_len(netuid: u16) -> u64 {
        ValidatorPruneLen::<T>::get(netuid)
    }
    pub fn set_validator_prune_len(netuid: u16, validator_prune_len: u64) {
        ValidatorPruneLen::<T>::insert(netuid, validator_prune_len);
        Self::deposit_event(Event::ValidatorPruneLenSet(netuid, validator_prune_len));
    }

    pub fn get_scaling_law_power(netuid: u16) -> u16 {
        ScalingLawPower::<T>::get(netuid)
    }
    pub fn set_scaling_law_power(netuid: u16, scaling_law_power: u16) {
        ScalingLawPower::<T>::insert(netuid, scaling_law_power);
        Self::deposit_event(Event::ScalingLawPowerSet(netuid, scaling_law_power));
    }

    pub fn get_max_weight_limit(netuid: u16) -> u16 {
        MaxWeightsLimit::<T>::get(netuid)
    }
    pub fn set_max_weight_limit(netuid: u16, max_weight_limit: u16) {
        MaxWeightsLimit::<T>::insert(netuid, max_weight_limit);
        Self::deposit_event(Event::MaxWeightLimitSet(netuid, max_weight_limit));
    }

    pub fn get_immunity_period(netuid: u16) -> u16 {
        ImmunityPeriod::<T>::get(netuid)
    }
    pub fn set_immunity_period(netuid: u16, immunity_period: u16) {
        ImmunityPeriod::<T>::insert(netuid, immunity_period);
        Self::deposit_event(Event::ImmunityPeriodSet(netuid, immunity_period));
    }

    pub fn get_min_allowed_weights(netuid: u16) -> u16 {
        MinAllowedWeights::<T>::get(netuid)
    }
    pub fn set_min_allowed_weights(netuid: u16, min_allowed_weights: u16) {
        MinAllowedWeights::<T>::insert(netuid, min_allowed_weights);
        Self::deposit_event(Event::MinAllowedWeightSet(netuid, min_allowed_weights));
    }

    pub fn get_max_allowed_uids(netuid: u16) -> u16 {
        MaxAllowedUids::<T>::get(netuid)
    }
    pub fn set_max_allowed_uids(netuid: u16, max_allowed: u16) {
        MaxAllowedUids::<T>::insert(netuid, max_allowed);
        Self::deposit_event(Event::MaxAllowedUidsSet(netuid, max_allowed));
    }

    pub fn get_kappa(netuid: u16) -> u16 {
        Kappa::<T>::get(netuid)
    }
    pub fn set_kappa(netuid: u16, kappa: u16) {
        Kappa::<T>::insert(netuid, kappa);
        Self::deposit_event(Event::KappaSet(netuid, kappa));
    }

    pub fn get_rho(netuid: u16) -> u16 {
        Rho::<T>::get(netuid)
    }
    pub fn set_rho(netuid: u16, rho: u16) {
        Rho::<T>::insert(netuid, rho);
    }

    pub fn get_activity_cutoff(netuid: u16) -> u16 {
        ActivityCutoff::<T>::get(netuid)
    }
    pub fn set_activity_cutoff(netuid: u16, activity_cutoff: u16) {
        ActivityCutoff::<T>::insert(netuid, activity_cutoff);
        Self::deposit_event(Event::ActivityCutoffSet(netuid, activity_cutoff));
    }

    // Registration Toggle utils
    pub fn get_network_registration_allowed(netuid: u16) -> bool {
        NetworkRegistrationAllowed::<T>::get(netuid)
    }
    pub fn set_network_registration_allowed(netuid: u16, registration_allowed: bool) {
        NetworkRegistrationAllowed::<T>::insert(netuid, registration_allowed);
        Self::deposit_event(Event::RegistrationAllowed(netuid, registration_allowed));
    }

    pub fn get_network_pow_registration_allowed(netuid: u16) -> bool {
        NetworkPowRegistrationAllowed::<T>::get(netuid)
    }
    pub fn set_network_pow_registration_allowed(netuid: u16, registration_allowed: bool) {
        NetworkPowRegistrationAllowed::<T>::insert(netuid, registration_allowed);
        Self::deposit_event(Event::PowRegistrationAllowed(netuid, registration_allowed));
    }

    pub fn get_target_registrations_per_interval(netuid: u16) -> u16 {
        TargetRegistrationsPerInterval::<T>::get(netuid)
    }
    pub fn set_target_registrations_per_interval(
        netuid: u16,
        target_registrations_per_interval: u16,
    ) {
        TargetRegistrationsPerInterval::<T>::insert(netuid, target_registrations_per_interval);
        Self::deposit_event(Event::RegistrationPerIntervalSet(
            netuid,
            target_registrations_per_interval,
        ));
    }

    pub fn get_burn_as_u64(netuid: u16) -> u64 {
        Burn::<T>::get(netuid)
    }
    pub fn set_burn(netuid: u16, burn: u64) {
        Burn::<T>::insert(netuid, burn);
    }

    pub fn get_min_burn_as_u64(netuid: u16) -> u64 {
        MinBurn::<T>::get(netuid)
    }
    pub fn set_min_burn(netuid: u16, min_burn: u64) {
        MinBurn::<T>::insert(netuid, min_burn);
        Self::deposit_event(Event::MinBurnSet(netuid, min_burn));
    }

    pub fn get_max_burn_as_u64(netuid: u16) -> u64 {
        MaxBurn::<T>::get(netuid)
    }
    pub fn set_max_burn(netuid: u16, max_burn: u64) {
        MaxBurn::<T>::insert(netuid, max_burn);
        Self::deposit_event(Event::MaxBurnSet(netuid, max_burn));
    }

    pub fn get_difficulty_as_u64(netuid: u16) -> u64 {
        Difficulty::<T>::get(netuid)
    }
    pub fn set_difficulty(netuid: u16, difficulty: u64) {
        Difficulty::<T>::insert(netuid, difficulty);
        Self::deposit_event(Event::DifficultySet(netuid, difficulty));
    }

    pub fn get_max_allowed_validators(netuid: u16) -> u16 {
        MaxAllowedValidators::<T>::get(netuid)
    }
    pub fn set_max_allowed_validators(netuid: u16, max_allowed_validators: u16) {
        MaxAllowedValidators::<T>::insert(netuid, max_allowed_validators);
        Self::deposit_event(Event::MaxAllowedValidatorsSet(
            netuid,
            max_allowed_validators,
        ));
    }

    pub fn get_bonds_moving_average(netuid: u16) -> u64 {
        BondsMovingAverage::<T>::get(netuid)
    }
    pub fn set_bonds_moving_average(netuid: u16, bonds_moving_average: u64) {
        BondsMovingAverage::<T>::insert(netuid, bonds_moving_average);
        Self::deposit_event(Event::BondsMovingAverageSet(netuid, bonds_moving_average));
    }

    pub fn get_max_registrations_per_block(netuid: u16) -> u16 {
        MaxRegistrationsPerBlock::<T>::get(netuid)
    }
    pub fn set_max_registrations_per_block(netuid: u16, max_registrations_per_block: u16) {
        MaxRegistrationsPerBlock::<T>::insert(netuid, max_registrations_per_block);
        Self::deposit_event(Event::MaxRegistrationsPerBlockSet(
            netuid,
            max_registrations_per_block,
        ));
    }

    pub fn get_subnet_owner(netuid: u16) -> T::AccountId {
        SubnetOwner::<T>::get(netuid)
    }
    pub fn get_subnet_owner_cut() -> u16 {
        SubnetOwnerCut::<T>::get()
    }
    pub fn set_subnet_owner_cut(subnet_owner_cut: u16) {
        SubnetOwnerCut::<T>::set(subnet_owner_cut);
        Self::deposit_event(Event::SubnetOwnerCutSet(subnet_owner_cut));
    }

    pub fn set_total_issuance(total_issuance: u64) {
        TotalIssuance::<T>::put(total_issuance);
    }

    pub fn get_rao_recycled(netuid: u16) -> u64 {
        RAORecycledForRegistration::<T>::get(netuid)
    }
    pub fn set_rao_recycled(netuid: u16, rao_recycled: u64) {
        RAORecycledForRegistration::<T>::insert(netuid, rao_recycled);
        Self::deposit_event(Event::RAORecycledForRegistrationSet(netuid, rao_recycled));
    }
    pub fn increase_rao_recycled(netuid: u16, inc_rao_recycled: u64) {
        let curr_rao_recycled = Self::get_rao_recycled(netuid);
        let rao_recycled = curr_rao_recycled.saturating_add(inc_rao_recycled);
        Self::set_rao_recycled(netuid, rao_recycled);
    }

    pub fn set_senate_required_stake_perc(required_percent: u64) {
        SenateRequiredStakePercentage::<T>::put(required_percent);
    }

    pub fn is_senate_member(hotkey: &T::AccountId) -> bool {
        T::SenateMembers::is_member(hotkey)
    }

    pub fn do_set_senate_required_stake_perc(
        origin: T::RuntimeOrigin,
        required_percent: u64,
    ) -> DispatchResult {
        ensure_root(origin)?;

        Self::set_senate_required_stake_perc(required_percent);
        Self::deposit_event(Event::SenateRequiredStakePercentSet(required_percent));
        Ok(())
    }

    pub fn is_subnet_owner(address: &T::AccountId) -> bool {
        SubnetOwner::<T>::iter_values().any(|owner| *address == owner)
    }
}
