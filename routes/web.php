<?php

use App\Http\Controllers\AccountController;
use App\Http\Controllers\AuthController;
use App\Http\Controllers\DashboardController;
use App\Http\Controllers\SocialiteController;
use Illuminate\Support\Facades\Route;
use Inertia\Inertia;

Route::get('/', function () {

    return Inertia::render('welcome/Welcome');
});

Route::middleware('guest')->group(function () {

    Route::get('login', [AuthController::class, 'login'])
        ->name('login');

    Route::get('auth/google', [SocialiteController::class, 'google'])
        ->name('auth.google');

    Route::get('auth/google/callback', [SocialiteController::class, 'googleCallback'])
        ->name('auth.google.callback');

    Route::get('auth/twitter', [SocialiteController::class, 'twitter'])
        ->name('auth.twitter');

    Route::get('auth/twitter/callback', [SocialiteController::class, 'twitterCallback'])
        ->name('auth.twitter.callback');
});

Route::middleware('auth')->group(function () {

    Route::get('logout', [AuthController::class, 'logout'])
        ->name('logout');
});

Route::middleware(['auth', 'verified'])->group(function () {

    Route::get('account', [AccountController::class, 'view'])
        ->name('account');

    Route::get('dashboard', [DashboardController::class, 'view'])
        ->name('dashboard');
});