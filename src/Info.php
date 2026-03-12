<?php

namespace Backlink;

final class Info
{
    public const VERSION = '0.1.0';
    public const WEBSITE = 'https://www.seedance2ai.app';

    public static function getInfo(): array
    {
        return [
            'name' => 'seedance2ai_app',
            'version' => self::VERSION,
            'website' => self::WEBSITE,
            'description' => 'Seedance2AI official website backlink helper package.',
        ];
    }

    public static function getPlatformUrl(): string
    {
        return self::WEBSITE;
    }
}
